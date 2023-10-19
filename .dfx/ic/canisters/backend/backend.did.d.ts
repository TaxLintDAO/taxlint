import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface BallotInfo { 'vote' : number, 'proposal_id' : [] | [NeuronId] }
export interface CanisterChange {
  'timestamp_nanos' : bigint,
  'canister_version' : bigint,
  'origin' : CanisterChangeOrigin,
  'details' : CanisterChangeDetails,
}
export type CanisterChangeDetails = { 'creation' : CreationRecord } |
  { 'code_deployment' : CodeDeploymentRecord } |
  { 'controllers_change' : CreationRecord } |
  { 'code_uninstall' : null };
export type CanisterChangeOrigin = { 'from_user' : FromUserRecord } |
  { 'from_canister' : FromCanisterRecord };
export interface CanisterInfoResponse {
  'controllers' : Array<Principal>,
  'module_hash' : [] | [Uint8Array | number[]],
  'recent_changes' : Array<CanisterChange>,
  'total_num_changes' : bigint,
}
export type CanisterInstallMode = { 'reinstall' : null } |
  { 'upgrade' : null } |
  { 'install' : null };
export interface CanisterStatusResponse {
  'status' : CanisterStatusType,
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : DefiniteCanisterSettings,
  'idle_cycles_burned_per_day' : bigint,
  'module_hash' : [] | [Uint8Array | number[]],
}
export type CanisterStatusType = { 'stopped' : null } |
  { 'stopping' : null } |
  { 'running' : null };
export interface CodeDeploymentRecord {
  'mode' : CanisterInstallMode,
  'module_hash' : Uint8Array | number[],
}
export interface CreationRecord { 'controllers' : Array<Principal> }
export type CustomResult1 = { 'Ok' : NeuronInfo } |
  { 'Err' : GovernanceError };
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export interface FromCanisterRecord {
  'canister_version' : [] | [bigint],
  'canister_id' : Principal,
}
export interface FromUserRecord { 'user_id' : Principal }
export interface GovernanceError {
  'error_message' : string,
  'error_type' : number,
}
export interface KnownNeuronData {
  'name' : string,
  'description' : [] | [string],
}
export interface NeuronId { 'id' : Uint8Array | number[] }
export interface NeuronInfo {
  'dissolve_delay_seconds' : bigint,
  'recent_ballots' : Array<BallotInfo>,
  'created_timestamp_seconds' : bigint,
  'state' : number,
  'stake_e8s' : bigint,
  'joined_community_fund_timestamp_seconds' : [] | [bigint],
  'retrieved_at_timestamp_seconds' : bigint,
  'known_neuron_data' : [] | [KnownNeuronData],
  'voting_power' : bigint,
  'age_seconds' : bigint,
}
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : UserProfile } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : [CustomResult1] } |
  { 'Err' : [RejectionCode, string] };
export type Result_3 = { 'Ok' : Array<WalletProfile> } |
  { 'Err' : Array<WalletProfile> };
export interface UserProfile {
  'owner' : Principal,
  'name' : string,
  'create_time' : bigint,
}
export interface WalletAddCommand {
  'from' : string,
  'name' : string,
  'address' : string,
}
export interface WalletProfile {
  'id' : bigint,
  'from' : string,
  'name' : string,
  'create_time' : bigint,
  'address' : string,
  'holder' : Principal,
}
export interface _SERVICE {
  'add_wallet' : ActorMethod<[WalletAddCommand], Result>,
  'auto_register_user' : ActorMethod<[], Result_1>,
  'create_and_install' : ActorMethod<[], string>,
  'delete_wallet' : ActorMethod<[bigint], Result>,
  'get_canister_info' : ActorMethod<[string], CanisterInfoResponse>,
  'get_canister_status' : ActorMethod<[string], CanisterStatusResponse>,
  'get_neuron_info' : ActorMethod<[bigint], Result_2>,
  'list_all_user' : ActorMethod<[], Array<UserProfile>>,
  'query_all_wallets' : ActorMethod<[], Result_3>,
  'user_quantity' : ActorMethod<[], number>,
  'whoami' : ActorMethod<[], Principal>,
}
