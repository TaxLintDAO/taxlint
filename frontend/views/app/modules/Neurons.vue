<template>
  <div class="nns-container">
    <q-table
      grid
      :loading="tableLoading"
      title="NNS"
      :rows="rows"
      :columns="columns"
      selection="multiple"
      :filter="filter"
      row-key="address"
    >
      <template v-slot:top>
        <div class="q-gutter-sm">
          <q-btn color="primary" @click="openDialog('add')"
            >Add NNS Neuron</q-btn
          >
          <el-tooltip effect="dark" placement="top-start">
            <template #content>
              Your Principal ID is {{ principal }}
              <q-icon
                name="content_copy"
                class="cursor-pointer"
                @click="copyPid()"
              />
            </template>
            <q-btn
              flat
              color="primary"
              label="How to import neuron"
              icon="lightbulb_outline"
              :href="DOCS_URL + NNS_HELP"
              target="_blank"
            />
          </el-tooltip>
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
                  <q-btn
                    v-if="props.row.from !== 'hotkey'"
                    flat
                    icon="more_vert"
                  >
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
                          <q-item-section @click="deleteItem(props.row.id)">
                            Delete
                          </q-item-section>
                        </q-item>
                      </q-list>
                    </q-menu>
                  </q-btn>
                  <q-badge
                    v-else
                    outline
                    color="secondary"
                    label="Hotkey Import"
                  />
                </div>
              </q-card-section>
              <q-list>
                <q-item v-for="col in props.cols" :key="col.name">
                  <q-item-section>
                    <q-item-label>{{ col.label }}</q-item-label>
                    <q-item-label caption>{{ col.value }}</q-item-label>
                  </q-item-section>
                </q-item>
                <!-- 有值才显示 -->
                <q-item v-if="props.row.neruonId">
                  <q-item-section>
                    <q-item-label> Neruon Id </q-item-label>
                    <q-item-label caption>
                      {{ props.row.neruonId }}
                    </q-item-label>
                  </q-item-section>
                </q-item>
                <q-item v-if="props.row.maturity || props.row.maturity === 0">
                  <q-item-section>
                    <q-item-label> Maturity </q-item-label>
                    <q-item-label caption>
                      {{ props.row.maturity }}
                    </q-item-label>
                  </q-item-section>
                </q-item>
                <q-item v-if="props.row.stakedMaturity">
                  <q-item-section>
                    <q-item-label> StakedMaturity </q-item-label>
                    <q-item-label caption>
                      {{ props.row.stakedMaturity }}
                    </q-item-label>
                  </q-item-section>
                </q-item>
              </q-list>
            </q-card-section>
          </q-card>
        </div>
      </template>
    </q-table>
    <q-dialog v-model="dialogVisible">
      <q-card style="min-width: 350px">
        <q-card-section>
          <div class="text-h6">
            {{ isEdit ? "Edit Your Neuron" : "Your Neuron" }}
          </div>
        </q-card-section>
        <q-card-section class="q-pt-none">
          <q-form @submit="onSubmit" ref="form" class="q-gutter-md">
            <q-input
              filled
              :disable="isEdit"
              v-model="neuron.address"
              label="Neuron Account ID *"
              hint="Enter Neuron Account ID"
              lazy-rules
              :rules="[
                (val) =>
                  (val && val.length > 0 && val.length === 64) ||
                  'Please enter Account ID',
                (val) =>
                  (val && !rows.some((item) => item.address === val)) ||
                  isEdit ||
                  'Can not add this neuron, neuron account duplicated',
              ]"
            />
            <q-input
              filled
              v-model="neuron.name"
              label="Neuron Name *"
              hint="Nickname"
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
import { DOCS_URL, NNS_HELP } from "@/api/constants/docs"
import {
  addUserNeuron,
  deleteUserNeuron,
  editUserNeuron,
  getUserNeuron,
} from "@/api/user"
import { useUserStore } from "@/stores/user"
import type { WalletInfo } from "@/types/user"
import { confirmDialog } from "@/utils/dialog"
import {
  showMessageError,
  showMessageSuccess,
  showResultError,
} from "@/utils/message"
import { getNNS } from "@/utils/nns"
import { QForm, copyToClipboard } from "quasar"
import { onMounted, ref } from "vue"

const userStore = useUserStore()

const dialogVisible = ref(false)
const filter = ref("") //搜索框
const principal = ref(useUserStore().principal) //用户
const columns = [
  {
    name: "address",
    required: true,
    label: "Neuron Account",
    field: (row) => row.address,
  },
  // { name: "id", label: "Neuron Id", field: "id" },
  // { name: "maturity", label: "Maturity", field: "maturity" },
  // { name: "stakedMaturity", label: "StakedMaturity", field: "stakedMaturity" },
]
const form = ref<QForm | null>(null)

const rows = ref<any[]>([])
const nnsNeruons = ref<any[]>([])
const loading = ref(false)
const tableLoading = ref(false)
const isEdit = ref(false) // dialog是否是edit功能，否就是add功能
const address = ref("")
const neuron = ref({
  id: 0n,
  address: "",
  principal_id: [] as string[], // 无值就用[]，而不是[""]，不然opt类型会报错
  from: "NNS", // neuron固定from值，不需要改变
  name: "",
})

onMounted(async () => {
  await getHotkeyWallet()
  getNeurons(false)
  // getSNS()
})

const resetNeuron = () => {
  //将值重置为初始值。
  neuron.value = {
    id: 0n,
    address: "",
    principal_id: [] as string[],
    from: "NNS",
    name: "",
  }
}

const openDialog = (action: string, itemInfo?: WalletInfo) => {
  if (action === "edit" && itemInfo) {
    isEdit.value = true
    address.value = itemInfo.address
    neuron.value = { ...itemInfo }
  } else {
    //不为edit就是add
    isEdit.value = false
    address.value = ""
    resetNeuron()
  }
  dialogVisible.value = true
}

const onSubmit = async () => {
  loading.value = true
  const validationSuccess = await form.value?.validate()
  if (validationSuccess) {
    if (isEdit.value) {
      await editItem()
    } else {
      await addItem()
    }
    dialogVisible.value = false
  }
  loading.value = false
}

const getNeurons = (isRefresh: boolean) => {
  //执行add，delete操作后刷新缓存，其他查询操作则不需要刷新缓存。
  getUserNeuron(isRefresh)
    .then((res) => {
      console.log("getNeurons", res)
      if (res.Ok) {
        rows.value = res.Ok.concat(nnsNeruons.value)
        console.log("getNeurons", rows.value)
      }
    })
    .finally(() => {
      tableLoading.value = false
    })
}

const addItem = async () => {
  const { address, name } = neuron.value
  const res = await addUserNeuron(address.trim(), name.trim())
  console.log("add", res)
  if (res.Ok) {
    rows.value.push({ ...neuron.value })
    resetNeuron()
    dialogVisible.value = false
    getNeurons(true)
  } else {
    showResultError(res)
  }
}

const editItem = async () => {
  const { id, name } = neuron.value
  console.log("edit", id, name)
  const res = await editUserNeuron(id, name)
  console.log("edit", res)
  if (res.Ok) {
    getNeurons(true)
  } else {
    showResultError(res)
  }
}

const deleteItem = (itemId: bigint) => {
  confirmDialog({
    title: "Delete Neuron",
    message:
      "Are you sure delete this Neuron? Delete Neuron will clear this Neuron history info",
    okMethod: (data) => {
      deleteUserNeuron(itemId).then((res) => {
        if (res.Ok) {
          getNeurons(true)
          showMessageSuccess("delete wallet success")
        }
      })
    },
  })
}

const getHotkeyWallet = async () => {
  tableLoading.value = true
  const res = await getNNS()
  nnsNeruons.value.push(...res)
}

const copyPid = () => {
  copyToClipboard(principal.value)
    .then(() => {
      showMessageSuccess(`copy ${principal.value} success`)
    })
    .catch(() => {
      showMessageError("copy failed")
    })
}
</script>

<style lang="scss">
.nns-container {
  .head-icon {
    width: 32px !important;
  }
  .q-table__top {
    padding: 0;
  }
}
</style>
