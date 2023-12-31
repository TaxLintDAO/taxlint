// 统一用户信息的结构
// 注意和types.ts中的ApiUserInfo统一格式
export class UserInfo {
  // id = 0; //用户id
  owner = "" // 用户principal，唯一
  name = "" // 用户自己设置的用户名
  create_at = 0 //注册时间
}

export interface UserInfoElement {
  owner?: string
  name?: string
}

export interface WalletInfo {
  id: bigint
  address: string
  principal_id: string[] //opt
  from: string //'NNS' | 'Plug' | 'Stoic' | 'AstorMe'
  name: string
  transactions: number
  last_transaction_time: number
  last_sync_time: number
}

export interface WalletHistory {
  price: number
  amount: number
  walletAmount: number
  timestamp: number
  walletValue: number
  type: string
}

export interface WalletTag {
  address: string
  name: string
  from: string
}

export interface NNSNeuron {
  neuronId: bigint
  address: string
  from: string
  maturity: number
  stakedMaturity: number
}

export interface Token {
  symbol: string
  balance: number
}

export interface Wallet {
  address: string
  name: string
  tokens: Token[]
}
