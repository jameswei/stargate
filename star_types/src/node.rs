//#[cfg(test)]
//mod protobuf_conversion_test;

use failure::prelude::*;
#[cfg(any(test, feature = "testing"))]
use proptest_derive::Arbitrary;
use crate::script_package::ChannelScriptPackage;
use libra_types::account_address::AccountAddress;
use protobuf::RepeatedField;
use libra_types::transaction::SignedTransactionWithProof;
use crypto::HashValue;
use std::convert::{TryFrom,TryInto};

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct OpenChannelRequest {
    pub remote_addr: AccountAddress,
    pub local_amount: u64,
    pub remote_amount: u64,
}

impl OpenChannelRequest {
    pub fn new(remote_addr: AccountAddress, local_amount: u64, remote_amount: u64) -> Self {
        Self {
            remote_addr,
            local_amount,
            remote_amount,
        }
    }
}

impl TryFrom<crate::proto::star_types::OpenChannelRequest> for OpenChannelRequest{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::OpenChannelRequest) -> Result<Self> {
        Ok(Self{
            remote_addr: value.remote_addr.try_into()?,
            local_amount: value.local_amount,
            remote_amount: value.remote_amount,
        })
    }
}

impl From<OpenChannelRequest> for crate::proto::star_types::OpenChannelRequest{

    fn from(value: OpenChannelRequest) -> Self {
        Self{
            remote_addr: value.remote_addr.into(),
            local_amount: value.local_amount,
            remote_amount: value.remote_amount,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct OpenChannelResponse {}

impl OpenChannelResponse {
    pub fn new() -> Self {
        OpenChannelResponse {}
    }
}

impl TryFrom<crate::proto::star_types::OpenChannelResponse> for OpenChannelResponse{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::OpenChannelResponse) -> Result<Self> {
        Ok(Self::new())
    }
}

impl From<OpenChannelResponse> for crate::proto::star_types::OpenChannelResponse{

    fn from(value: OpenChannelResponse) -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct PayRequest {
    pub remote_addr: AccountAddress,
    pub amount: u64,
}

impl PayRequest {
    pub fn new(remote_addr: AccountAddress, amount: u64) -> Self {
        PayRequest {
            remote_addr,
            amount,
        }
    }
}

impl TryFrom<crate::proto::star_types::PayRequest> for PayRequest{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::PayRequest) -> Result<Self> {
        Ok(Self{
            remote_addr: value.remote_addr.try_into()?,
            amount: value.amount,
        })
    }
}

impl From<PayRequest> for crate::proto::star_types::PayRequest{

    fn from(value: PayRequest) -> Self {
        Self{
            remote_addr: value.remote_addr.into(),
            amount: value.amount,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct PayResponse {}

impl PayResponse {
    pub fn new() -> Self {
        PayResponse {}
    }
}


impl TryFrom<crate::proto::star_types::PayResponse> for PayResponse{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::PayResponse) -> Result<Self> {
        Ok(Self::new())
    }
}

impl From<PayResponse> for crate::proto::star_types::PayResponse{

    fn from(value: PayResponse) -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct ConnectRequest {
    pub remote_addr: AccountAddress,
    pub remote_ip: String,
}

impl ConnectRequest {
    pub fn new(remote_addr: AccountAddress, remote_ip: String) -> Self {
        Self {
            remote_addr,
            remote_ip,
        }
    }
}

impl TryFrom<crate::proto::star_types::ConnectRequest> for ConnectRequest{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::ConnectRequest) -> Result<Self> {
        Ok(Self{
            remote_addr: value.remote_addr.try_into()?,
            remote_ip: value.remote_ip,
        })
    }
}

impl From<ConnectRequest> for crate::proto::star_types::ConnectRequest{

    fn from(value: ConnectRequest) -> Self {
        Self{
            remote_addr: value.remote_addr.into(),
            remote_ip: value.remote_ip,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct ConnectResponse {}

impl ConnectResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryFrom<crate::proto::star_types::ConnectResponse> for ConnectResponse{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::ConnectResponse) -> Result<Self> {
        Ok(Self::new())
    }
}

impl From<ConnectResponse> for crate::proto::star_types::ConnectResponse{

    fn from(value: ConnectResponse) -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct DepositRequest {
    pub remote_addr: AccountAddress,
    pub local_amount: u64,
    pub remote_amount: u64,
}

impl DepositRequest {
    pub fn new(remote_addr: AccountAddress, local_amount: u64, remote_amount: u64) -> Self {
        Self {
            remote_addr,
            local_amount,
            remote_amount,
        }
    }
}

impl TryFrom<crate::proto::star_types::DepositRequest> for DepositRequest{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::DepositRequest) -> Result<Self> {
        Ok(Self{
            remote_addr: value.remote_addr.try_into()?,
            local_amount: value.local_amount,
            remote_amount: value.remote_amount,
        })
    }
}

impl From<DepositRequest> for crate::proto::star_types::DepositRequest{

    fn from(value: DepositRequest) -> Self {
        Self{
            remote_addr: value.remote_addr.into(),
            local_amount: value.local_amount,
            remote_amount: value.remote_amount,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct DepositResponse {}

impl DepositResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryFrom<crate::proto::star_types::DepositResponse> for DepositResponse{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::DepositResponse) -> Result<Self> {
        Ok(Self::new())
    }
}

impl From<DepositResponse> for crate::proto::star_types::DepositResponse{

    fn from(value: DepositResponse) -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct WithdrawRequest {
    pub remote_addr: AccountAddress,
    pub local_amount: u64,
    pub remote_amount: u64,
}

impl WithdrawRequest {
    pub fn new(remote_addr: AccountAddress, local_amount: u64, remote_amount: u64) -> Self {
        Self {
            remote_addr,
            local_amount,
            remote_amount,
        }
    }
}

impl TryFrom<crate::proto::star_types::WithdrawRequest> for WithdrawRequest{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::WithdrawRequest) -> Result<Self> {
        Ok(Self{
            remote_addr: value.remote_addr.try_into()?,
            local_amount: value.local_amount,
            remote_amount: value.remote_amount,
        })
    }
}

impl From<WithdrawRequest> for crate::proto::star_types::WithdrawRequest{

    fn from(value: WithdrawRequest) -> Self {
        Self{
            remote_addr: value.remote_addr.into(),
            local_amount: value.local_amount,
            remote_amount: value.remote_amount,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct WithdrawResponse {}

impl WithdrawResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryFrom<crate::proto::star_types::WithdrawResponse> for WithdrawResponse{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::WithdrawResponse) -> Result<Self> {
        Ok(Self::new())
    }
}

impl From<WithdrawResponse> for crate::proto::star_types::WithdrawResponse{

    fn from(value: WithdrawResponse) -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct ChannelBalanceRequest {
    pub remote_addr: AccountAddress,
}

impl ChannelBalanceRequest {
    pub fn new(remote_addr: AccountAddress) -> Self {
        Self { remote_addr }
    }
}

impl TryFrom<crate::proto::star_types::ChannelBalanceRequest> for ChannelBalanceRequest{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::ChannelBalanceRequest) -> Result<Self> {
        Ok(Self{
            remote_addr: value.remote_addr.try_into()?,
        })
    }
}

impl From<ChannelBalanceRequest> for crate::proto::star_types::ChannelBalanceRequest{

    fn from(value: ChannelBalanceRequest) -> Self {
        Self{
            remote_addr: value.remote_addr.into(),
        }
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct ChannelBalanceResponse {
    pub balance: u64,
}

impl ChannelBalanceResponse {
    pub fn new(balance: u64) -> Self {
        Self { balance }
    }
}

impl TryFrom<crate::proto::star_types::ChannelBalanceResponse> for ChannelBalanceResponse{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::ChannelBalanceResponse) -> Result<Self> {
        Ok(Self{
            balance: value.balance,
        })
    }
}

impl From<ChannelBalanceResponse> for crate::proto::star_types::ChannelBalanceResponse{

    fn from(value: ChannelBalanceResponse) -> Self {
        Self{
            balance: value.balance,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstallChannelScriptPackageRequest {
    pub channel_script_package: ChannelScriptPackage,
}

impl InstallChannelScriptPackageRequest{
    pub fn new(channel_script_package: ChannelScriptPackage) -> Self {
        Self {channel_script_package}
    }
}

impl TryFrom<crate::proto::star_types::InstallChannelScriptPackageRequest> for InstallChannelScriptPackageRequest{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::InstallChannelScriptPackageRequest) -> Result<Self> {
        Ok(Self{
            channel_script_package: value.channel_script_package.ok_or_else(|| format_err!("Missing channel_script_package"))?.try_into()?,
        })
    }
}

impl From<InstallChannelScriptPackageRequest> for crate::proto::star_types::InstallChannelScriptPackageRequest{

    fn from(value: InstallChannelScriptPackageRequest) -> Self {
        Self{
            channel_script_package: Some(value.channel_script_package.into()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct InstallChannelScriptPackageResponse {}

impl InstallChannelScriptPackageResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl TryFrom<crate::proto::star_types::InstallChannelScriptPackageResponse> for InstallChannelScriptPackageResponse{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::InstallChannelScriptPackageResponse) -> Result<Self> {
        Ok(Self::new())
    }
}

impl From<InstallChannelScriptPackageResponse> for crate::proto::star_types::InstallChannelScriptPackageResponse{

    fn from(value: InstallChannelScriptPackageResponse) -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeployModuleRequest {
    pub module_bytes: Vec<u8>,
}

impl DeployModuleRequest{
    pub fn new(module_bytes: Vec<u8>) -> Self {
        Self {module_bytes}
    }
}

impl TryFrom<crate::proto::star_types::DeployModuleRequest> for DeployModuleRequest{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::DeployModuleRequest) -> Result<Self> {
        Ok(Self{
            module_bytes: value.module_bytes,
        })
    }
}

impl From<DeployModuleRequest> for crate::proto::star_types::DeployModuleRequest{

    fn from(value: DeployModuleRequest) -> Self {
        Self{
            module_bytes: value.module_bytes,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct DeployModuleResponse {
    pub transaction_with_proof :SignedTransactionWithProof,
}

impl DeployModuleResponse {
    pub fn new( transaction_with_proof :SignedTransactionWithProof) -> Self {
        Self {transaction_with_proof}
    }
}

impl TryFrom<crate::proto::star_types::DeployModuleResponse> for DeployModuleResponse{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::DeployModuleResponse) -> Result<Self> {
        Ok(Self{
            transaction_with_proof: value.transaction_with_proof.ok_or_else(|| format_err!("Missing transaction_with_proof"))?.try_into()?,
        })
    }
}

impl From<DeployModuleResponse> for crate::proto::star_types::DeployModuleResponse{

    fn from(value: DeployModuleResponse) -> Self {
        Self{
            transaction_with_proof: Some(value.transaction_with_proof.into()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecuteScriptRequest{
    pub remote_addr:AccountAddress,
    pub package_name:String,
    pub script_name:String,
    pub args:Vec<Vec<u8>>,
}

impl ExecuteScriptRequest {
    pub fn new(remote_addr:AccountAddress,package_name:String, script_name:String, args:Vec<Vec<u8>>)->Self{
        Self{
            remote_addr,
            package_name,
            script_name,
            args,
        }
    }
}


impl TryFrom<crate::proto::star_types::ExecuteScriptRequest> for ExecuteScriptRequest{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::ExecuteScriptRequest) -> Result<Self> {
        Ok(Self{
            remote_addr:value.remote_addr.try_into()?,
            package_name:value.package_name,
            script_name:value.script_name,
            args:value.args.to_vec(),
        })
    }
}

impl From<ExecuteScriptRequest> for crate::proto::star_types::ExecuteScriptRequest{

    fn from(value: ExecuteScriptRequest) -> Self {
        Self{
            remote_addr: value.remote_addr.into(),
            package_name: value.package_name,
            script_name: value.script_name,
            args: value.args,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(Arbitrary))]
pub struct ExecuteScriptResponse{
    pub hash_value:HashValue,
}

impl ExecuteScriptResponse{
    pub fn new(hash_value:HashValue)->Self{
        Self{hash_value}
    }
}

impl TryFrom<crate::proto::star_types::ExecuteScriptResponse> for ExecuteScriptResponse{
    type Error = Error;

    fn try_from(value: crate::proto::star_types::ExecuteScriptResponse) -> Result<Self> {
        Ok(Self::new(HashValue::from_slice(value.hash_value.as_slice())?))
    }
}

impl From<ExecuteScriptResponse> for crate::proto::star_types::ExecuteScriptResponse{

    fn from(value: ExecuteScriptResponse) -> Self {
        Self{
            hash_value: value.hash_value.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests{

    #[test]
    fn test_compile(){
        println!("it work");
    }
}