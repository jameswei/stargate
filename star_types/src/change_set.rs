use std::ops::Index;
use std::slice::SliceIndex;

use serde::{Deserialize, Serialize};

use failure::prelude::*;
use proto_conv::{FromProto, IntoProto};
use crate::access_path::{AccessPath, Accesses};
use vm_runtime_types::value::Value;
use std::mem;

#[derive(Clone, Debug)]
pub enum ChangeOp {
    None,
    Plus(u64),
    Minus(u64),
    Update(Value),
    Deletion,
}

impl ChangeOp {
    pub fn is_none(&self) -> bool {
        match self {
            ChangeOp::None => true,
            _ => false
        }
    }

    pub fn is_deletion(&self) -> bool {
        match self {
            ChangeOp::Deletion => true,
            _ => false
        }
    }

    pub fn as_plus(&self) -> Option<u64> {
        match self {
            ChangeOp::Plus(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_minus(&self) -> Option<u64> {
        match self {
            ChangeOp::Minus(value) => Some(*value),
            _ => None
        }
    }

    /// return old value.
    pub fn merge_with(&mut self, other: &ChangeOp) ->Result<ChangeOp>{
        let mut change_op = Self::merge(self, other)?;
        Ok(mem::replace(self, change_op))
    }

    pub fn merge(first: &ChangeOp, second: &ChangeOp) -> Result<ChangeOp> {
        match first {
            ChangeOp::None => Ok(second.clone()),
            ChangeOp::Plus(first_value) => {
                match second {
                    ChangeOp::None => Ok(ChangeOp::Plus(*first_value)),
                    ChangeOp::Plus(second_value) => Ok(ChangeOp::Plus(first_value + second_value)),
                    ChangeOp::Minus(second_value) => {
                        if first_value == second_value {
                            Ok(ChangeOp::None)
                        } else if first_value > second_value {
                            Ok(ChangeOp::Plus(first_value - second_value))
                        } else {
                            Ok(ChangeOp::Minus(second_value - first_value))
                        }
                    }
                    _ => bail!("can not merge  ChangeOp:{:?},{:?}", first, second),
                }
            }
            ChangeOp::Minus(first_value) => {
                match second {
                    ChangeOp::None => Ok(ChangeOp::Minus(*first_value)),
                    ChangeOp::Plus(second_value) => {
                        if first_value == second_value {
                            Ok(ChangeOp::None)
                        } else if first_value > second_value {
                            Ok(ChangeOp::Minus(first_value - second_value))
                        } else {
                            Ok(ChangeOp::Plus(second_value - first_value))
                        }
                    }
                    ChangeOp::Minus(second_value) => Ok(ChangeOp::Minus(first_value + second_value)),
                    _ => bail!("can not merge  ChangeOp:{:?},{:?}", first, second),
                }
            }
            ChangeOp::Update(first_value) => {
                match second {
                    ChangeOp::None => Ok(ChangeOp::Update(first_value.clone())),
                    ChangeOp::Update(second_value) => Ok(ChangeOp::Update(second_value.clone())),
                    ChangeOp::Deletion => Ok(ChangeOp::Deletion),
                    _ => bail!("can not merge  ChangeOp:{:?},{:?}", first, second),
                }
            }
            ChangeOp::Deletion => {
                match second {
                    ChangeOp::None => Ok(ChangeOp::Deletion),
                    ChangeOp::Deletion => Ok(ChangeOp::Deletion),
                    ChangeOp::Update(second_value) => Ok(ChangeOp::Update(second_value.clone())),
                    _ => bail!("can not merge  ChangeOp:{:?},{:?}", first, second),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChangeSet(ChangeSetMut);

impl ChangeSet {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> ::std::slice::Iter<(AccessPath, FieldChanges)> {
        self.into_iter()
    }

    #[inline]
    pub fn into_mut(self) -> ChangeSetMut {
        self.0
    }

    pub fn merge(first: &ChangeSet, second: &ChangeSet) -> Result<ChangeSet> {
        ChangeSetMut::merge(&first.0, &second.0).and_then(|change_set| change_set.freeze())
    }
}

#[derive(Clone,Debug)]
pub struct FieldChanges(Vec<(Accesses, ChangeOp)>);

impl FieldChanges{

    pub fn empty() -> Self{
        Self(vec![])
    }

    pub fn new(changes: Vec<(Accesses, ChangeOp)>) -> Self{
        Self(changes)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> ::std::slice::Iter<(Accesses, ChangeOp)> {
        self.into_iter()
    }

    pub fn append(&mut self, other:&mut FieldChanges){
        self.0.append(&mut other.0)
    }

    pub fn push(&mut self, item:(Accesses, ChangeOp)){
        self.0.push(item)
    }

    pub fn get_change(&self, accesses: &Accesses) -> Option<&ChangeOp>{
        self.iter().find(|(ac, _)| ac == accesses).map(|(_, change)| change)
    }

    pub fn get_change_mut(&mut self, accesses: &Accesses) -> Option<&mut ChangeOp>{
        self.0.iter_mut().find(|(ac, _)| ac == accesses).map(|(_, change)| change)
    }

    pub fn merge_with(&mut self, other: &FieldChanges) -> Result<FieldChanges>{
        let mut changes = Self::merge(self, other)?;
        Ok(mem::replace(self,changes))
    }

    pub fn merge(first: &FieldChanges, second: &FieldChanges) -> Result<FieldChanges>{
        let mut changes = first.clone();
        for (accesses, second_change_op) in second{
            match changes.get_change_mut(&accesses){
                Some(change_op) => {
                    change_op.merge_with(second_change_op)?;
                },
                None => {
                    changes.push((accesses.clone(), second_change_op.clone()))
                }
            }
        }
        Ok(changes)
    }

    pub fn filter_none(&mut self){
        let mut changes = FieldChanges::new(self.0.iter().filter(|(_,change_op)|!change_op.is_none()).cloned().collect());
        mem::replace(self, changes);
    }
}


impl ::std::iter::IntoIterator for FieldChanges {
    type Item = (Accesses, ChangeOp);
    type IntoIter = ::std::vec::IntoIter<(Accesses, ChangeOp)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


impl<'a> IntoIterator for &'a FieldChanges {
    type Item = &'a (Accesses, ChangeOp);
    type IntoIter = ::std::slice::Iter<'a, (Accesses, ChangeOp)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}


impl Index<usize> for FieldChanges {
    type Output = (Accesses, ChangeOp);

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}



#[derive(Clone, Debug, Default)]
pub struct ChangeSetMut {
    change_set: Vec<(AccessPath, FieldChanges)>,
}

impl ChangeSetMut {
    pub fn new(change_set: Vec<(AccessPath, FieldChanges)>) -> Self {
        Self { change_set }
    }

    pub fn push(&mut self, item: (AccessPath, FieldChanges)) {
        self.change_set.push(item);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.change_set.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.change_set.is_empty()
    }

    pub fn freeze(self) -> Result<ChangeSet> {
        // TODO: add structural validation
        Ok(ChangeSet(self))
    }

    pub fn get_changes(&self, access_path: &AccessPath) -> Option<&FieldChanges> {
        self.change_set.iter().find(|(ap, _)| ap == access_path).map(|(_, change)| change)
    }

    pub fn get_changes_mut(&mut self, access_path: &AccessPath) -> Option<&mut FieldChanges> {
        self.change_set.iter_mut().find(|(ap, _)| ap == access_path).map(|(_, change)| change)
    }

    /// return old change set
    pub fn merge_with(&mut self, other:&ChangeSetMut) -> Result<ChangeSetMut> {
        let mut change_set = Self::merge(self, other)?;
        Ok(mem::replace(self, change_set))
    }

    pub fn merge(first: &ChangeSetMut, second: &ChangeSetMut) -> Result<ChangeSetMut> {
        let mut change_set = first.clone();
        for (ap, second_change) in &second.change_set {
            match change_set.get_changes_mut(ap) {
                Some(first_change) => {
                    first_change.merge_with(second_change)?;
                }
                None => {
                    change_set.push((ap.clone(), second_change.clone()));
                }
            }
        }
        Ok(change_set)
    }
}

impl Index<usize> for ChangeSetMut {
    type Output = (AccessPath, FieldChanges);

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.change_set[index]
    }
}


impl ::std::iter::FromIterator<(AccessPath, FieldChanges)> for ChangeSetMut {
    fn from_iter<I: IntoIterator<Item=(AccessPath, FieldChanges)>>(iter: I) -> Self {
        let mut ws = ChangeSetMut::default();
        for write in iter {
            ws.push((write.0, write.1));
        }
        ws
    }
}

impl<'a> IntoIterator for &'a ChangeSet {
    type Item = &'a (AccessPath, FieldChanges);
    type IntoIter = ::std::slice::Iter<'a, (AccessPath, FieldChanges)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.change_set.iter()
    }
}

impl ::std::iter::IntoIterator for ChangeSet {
    type Item = (AccessPath, FieldChanges);
    type IntoIter = ::std::vec::IntoIter<(AccessPath, FieldChanges)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.change_set.into_iter()
    }
}
