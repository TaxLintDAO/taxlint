<template xmlns:v-slot="http://www.w3.org/1999/XSL/Transform">
  <div class="wallet-container">
    <q-table
      grid
      :loading="tableLoading"
      title="Wallets"
      :rows="rows"
      :columns="columns"
      selection="multiple"
      v-model:selected="selected"
      :filter="filter"
      row-key="address"
    >
      <template v-slot:top>
        <div class="q-gutter-sm">
          <q-btn color="primary" @click="openDialog('add')">Add Wallet</q-btn>
          <q-btn color="secondary" @click="syncAllWallet()" icon="cached"
            >Sync All Wallet</q-btn
          >
        </div>
        <q-space />
        <q-input
          borderless
          dense
          debounce="300"
          v-model="filter"
          placeholder="Search"
        >
          <template v-slot:append>
            <q-icon name="search" />
          </template>
        </q-input>
      </template>
      <template v-slot:item="props">
        <div
          class="q-pa-xs col-xs-12 col-sm-6 col-md-4 col-lg-3 grid-style-transition"
        >
          <q-card>
            <q-card-section>
              <q-card-section>
                <div class="row justify-between items-center">
                  <div class="flex q-gutter-xs">
                    <img
                      class="head-icon"
                      src="@/assets/dfinity.svg"
                      alt="NNS Icon"
                    />
                    <div class="text-h6">{{ props.row.name }}</div>
                  </div>
                  <q-btn flat icon="more_vert">
                    <q-menu>
                      <q-list style="min-width: 100px">
                        <q-item clickable v-close-popup="true">
                          <q-item-section
                            @click="openDialog('edit', props.row)"
                          >
                            Edit
                          </q-item-section>
                        </q-item>
                        <q-item clickable v-close-popup="true">
                          <q-item-section @click="deleteWallet(props.row.id)">
                            Delete
                          </q-item-section>
                        </q-item>
                      </q-list>
                    </q-menu>
                  </q-btn>
                </div>
              </q-card-section>
              <q-list>
                <!-- principal只有有值才显示 -->
                <q-item v-if="props.row.principal_id.length > 0">
                  <q-item-section>
                    <q-item-label> Principal </q-item-label>
                    <q-item-label caption>
                      {{ props.row.principal_id[0] }}
                    </q-item-label>
                  </q-item-section>
                </q-item>
                <q-item v-for="col in props.cols" :key="col.name">
                  <q-item-section>
                    <q-item-label>{{ col.label }}</q-item-label>
                    <q-item-label v-if="col.name === 'address'" caption>
                      <router-link :to="'/app/transactions/' + col.value">
                        {{ col.value }}
                      </router-link>
                    </q-item-label>
                    <q-item-label v-else caption>{{ col.value }}</q-item-label>
                  </q-item-section>
                </q-item>
              </q-list>
            </q-card-section>
          </q-card>
        </div>
      </template>
    </q-table>
    <q-dialog v-model="walletDialogVisible">
      <q-card style="min-width: 350px">
        <q-card-section>
          <div class="text-h6">
            {{ isEdit ? "Edit Your Wallet" : "Your Wallet" }}
          </div>
        </q-card-section>
        <q-card-section class="q-pt-none">
          <q-form @submit="onSubmit" ref="walletForm" class="q-gutter-md">
            <q-input
              filled
              :disable="isEdit"
              v-model="address"
              label="Wallet Address *"
              hint="Enter Principal ID or Account ID"
              lazy-rules
              :rules="[
                (val) =>
                  (val &&
                    val.length > 0 &&
                    (val.length === 63 || val.length === 64)) ||
                  'Please enter Principal ID or Account ID',
                (val) =>
                  (val && !rows.some((item) => item.address === val)) ||
                  isEdit ||
                  'Can not add this wallet, address duplicated',
              ]"
            />
            <q-input
              filled
              v-if="addressIsPrincipal"
              label="Wallet Account ID"
              v-model="wallet.address"
            />
            <q-select
              filled
              v-model="wallet.from"
              :options="froms"
              label="From"
            />
            <q-input
              filled
              v-model="wallet.name"
              label="Wallet Name *"
              hint="Identify your wallet quickly"
              lazy-rules
              :rules="[
                (val) => (val && val.length > 0) || 'Please type something',
              ]"
            />
            <div class="q-gutter-sm justify-end flex">
              <q-btn flat label="Cancel" v-close-popup="true" />
              <q-btn
                v-if="isEdit"
                :loading="loading"
                label="Update"
                type="submit"
                color="primary"
              />
              <q-btn
                v-else
                :loading="loading"
                label="Submit"
                type="submit"
                color="primary"
              />
            </div>
          </q-form>
        </q-card-section>
      </q-card>
    </q-dialog>
  </div>
</template>

<script lang="ts" setup>
import { getICPTransactions } from "@/api/rosetta"
import {
  addUserWallet,
  deleteUserWallet,
  editUserWallet,
  getUserWallet,
  syncWallet,
} from "@/api/user"
import type { WalletInfo } from "@/types/user"
import { isPrincipal, p2a } from "@/utils/common"
import { confirmDialog } from "@/utils/dialog"
import { showMessageSuccess, showResultError } from "@/utils/message"
import type { QForm } from "quasar"
import { onMounted, ref, watch } from "vue"

const columns = [
  {
    name: "address",
    required: true,
    label: "Address",
    field: (row) => row.address,
  },
  { name: "from", label: "From", field: "from" },
  { name: "name", label: "Name", field: "name" },
  { name: "transactions", label: "Transactions", field: "transactions" },
]
const froms = ["NNS", "Plug", "Stoic", "AstorMe"]
const walletDialogVisible = ref(false)
const loading = ref(false)
const tableLoading = ref(false)
const filter = ref("") // 搜索框
const selected = ref([]) // 当前选中的对象们
const address = ref("") // 当前用户输入的地址，可能是principal ID，也可能是account ID
const addressIsPrincipal = ref(false) // 是否是principal，关系到某些字段的显示
const isEdit = ref(false) // dialog是否是edit功能，否就是add功能

const wallet = ref({
  id: 0n,
  address: "",
  principal_id: [] as string[], // 无值就用[]，而不是[""]，不然opt类型会报错
  from: "NNS",
  name: "",
  transactions: 0,
  last_transaction_time: 0,
  last_sync_time: 0,
})
const walletPrototype = {
  id: 0n,
  address: "",
  principal_id: [] as string[],
  from: "NNS",
  name: "",
  transactions: 0,
  last_transaction_time: 0,
  last_sync_time: 0,
}
const walletForm = ref<QForm | null>(null)

const rows = ref<WalletInfo[]>([])

onMounted(() => {
  getWallets(false)
})

const syncAllWallet = () => {
  rows.value.forEach((row, index) => {
    console.log(`Row ${index + 1}`, row)
    getICPTransactions({ address: row.address, name: "", from: "" }, true).then(
      (res) => {
        //将钱包数据同步
        syncWallet(row.id, res.transactions)
      },
    )
  })
}

// 识别用户输入的地址属于principal ID还是account ID
watch(address, () => {
  identifyAddress()
})

// 识别用户输入的地址属于principal ID还是account ID
const identifyAddress = () => {
  addressIsPrincipal.value = isPrincipal(address.value)
  if (addressIsPrincipal.value) {
    wallet.value.address = p2a(address.value)
    wallet.value.principal_id.push(address.value)
  } else {
    wallet.value.address = address.value
  }
}

const getWallets = (isRefresh: boolean) => {
  tableLoading.value = true
  //执行add，delete操作后刷新缓存，其他查询操作则不需要刷新缓存。
  getUserWallet(isRefresh)
    .then((res) => {
      console.log("getUserWallet", res)
      if (res.Ok) {
        rows.value = res.Ok
        for (const row of rows.value) {
          try {
            row.transactions = 0
            getICPTransactions(
              { address: row.address, name: "", from: "" },
              false,
            ).then((res) => {
              // 将查询得到的transactions绑定回原数组中的transactions
              row.transactions = res.total
            })
          } catch (error) {
            // 处理错误情况
            console.error(`查询地址 ${row.address} 的交易时出错:`, error)
          }
        }
      }
    })
    .finally(() => {
      tableLoading.value = false
    })
}

const onSubmit = async () => {
  loading.value = true
  const validationSuccess = await walletForm.value?.validate()
  if (validationSuccess) {
    if (isEdit.value) {
      await editWallet()
    } else {
      await addWallet()
    }
    walletDialogVisible.value = false
  } else {
    // 数据验证失败
    // 用户至少输入了一个无效值
  }
  loading.value = false
}

const openDialog = (action: string, itemInfo?: WalletInfo) => {
  if (action === "edit" && itemInfo) {
    isEdit.value = true
    address.value = itemInfo.address
    wallet.value = { ...itemInfo }
  } else {
    //不为edit就是add
    isEdit.value = false
    address.value = ""
    wallet.value = { ...walletPrototype }
  }
  walletDialogVisible.value = true
}

const addWallet = async () => {
  const { address, name, from, principal_id } = wallet.value
  const res = await addUserWallet(
    address.trim(),
    name.trim(),
    from,
    principal_id,
  )
  if (res.Ok) {
    rows.value.push({ ...wallet.value })
    wallet.value = { ...walletPrototype }
    walletDialogVisible.value = false
    getWallets(true)
  } else {
    showResultError(res)
  }
}

const editWallet = async () => {
  const { id, from, name } = wallet.value
  const res = await editUserWallet(id, from, name)
  if (res.Ok) {
    getWallets(true)
  } else {
    showResultError(res)
  }
}

const deleteWallet = (walletId: bigint) => {
  confirmDialog({
    title: "Delete Wallet",
    message:
      "Are you sure delete this wallet? Delete wallet will clear this wallet history info",
    okMethod: (data) => {
      deleteUserWallet(walletId).then((res) => {
        if (res.Ok) {
          getWallets(true)
          showMessageSuccess("delete wallet success")
        }
      })
    },
  })
}
</script>

<style lang="scss">
.wallet-container {
  .grid-style-transition {
    transition: transform 0.28s, background-color 0.28s;
  }
  .head-icon {
    width: 32px !important;
  }
  .text-caption {
    font-size: 0.9rem;
  }
  .q-table__top {
    padding: 0;
  }
}
</style>
