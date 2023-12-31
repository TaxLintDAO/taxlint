<template>
  <div class="dashboard-container">
    DashBoard
    <!-- 为 ECharts 准备一个定义了宽高的 DOM -->
    <div class="row q-gutter-md">
      <div class="col-7" ref="echartsContainer" style="height: 400px"></div>
      <div class="col-4">
        <div class="q-pa-md" style="max-width: 300px">
          <q-input filled v-model="date" mask="date" :rules="['date']">
            <template v-slot:append>
              <q-icon name="event" class="cursor-pointer">
                <q-popup-proxy
                  ref="qDateProxy"
                  cover
                  transition-show="scale"
                  transition-hide="scale"
                >
                  <q-date v-model="date">
                    <div class="row items-center justify-end">
                      <q-btn
                        v-close-popup="true"
                        label="Close"
                        color="primary"
                        flat
                      />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
        </div>
        <q-card flat bordered>
          <q-item>
            <q-item-section>
              <q-item-label caption> BREAKDOWN </q-item-label>
              <q-list>
                <q-item clickable v-ripple="true">
                  <q-item-section>Received</q-item-section>
                  <q-item-section side>$ {{ received }}</q-item-section>
                </q-item>
                <q-item clickable v-ripple="true">
                  <q-item-section>Sent</q-item-section>
                  <q-item-section side>$ {{ sent }}</q-item-section>
                </q-item>
                <q-item clickable v-ripple="true">
                  <q-item-section>Gains</q-item-section>
                  <q-item-section side>$ {{ gains }}</q-item-section>
                </q-item>
              </q-list>
            </q-item-section>
          </q-item>
        </q-card>
      </div>
    </div>
    <div class="row">
      <q-card flat bordered>
        <q-item>
          <q-item-section>
            <q-item-label caption style="font-size: 1rem">
              Holdings
            </q-item-label>
            <q-table
              :rows="rows"
              :columns="columns"
              row-key="name"
              :rowsPerPageOptions="[0]"
              hide-pagination
              flat
            >
              <template v-slot:header="props">
                <q-tr :props="props">
                  <q-th auto-width />
                  <q-th
                    v-for="col in props.cols"
                    :key="col.name"
                    :props="props"
                  >
                    {{ col.label }}
                  </q-th>
                </q-tr>
              </template>

              <template v-slot:body="props">
                <q-tr :props="props">
                  <q-td auto-width>
                    <q-btn
                      size="sm"
                      color="accent"
                      round
                      dense
                      @click="props.expand = !props.expand"
                      :icon="props.expand ? 'remove' : 'add'"
                    />
                  </q-td>
                  <q-td
                    v-for="col in props.cols"
                    :key="col.name"
                    :props="props"
                  >
                    {{ col.value }}
                  </q-td>
                </q-tr>
                <q-tr v-show="props.expand" :props="props" no-hover>
                  <q-td colspan="100%">
                    <div class="text-left">
                      <Progress
                        :wallets="wallets"
                        :symbol="props.row.token"
                        :totalBalance="icpBalance"
                      />
                    </div>
                  </q-td>
                </q-tr>
              </template>
            </q-table>
          </q-item-section>
        </q-item>
      </q-card>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getICPBalance, getWalletHistory } from "@/api/rosetta"
import { getICPNowPrice } from "@/api/token"
import { getUserWallet } from "@/api/user"
import Progress from "@/components/Progress.vue"
import type { Wallet, WalletHistory } from "@/types/user"
import { showMessageError } from "@/utils/message"
import * as echarts from "echarts"
import { onMounted, ref, watch } from "vue"

const echartsContainer = ref<null>(null)
const date = ref("2023/01/01")
const totalHistory = ref<WalletHistory[]>([])
const received = ref(0)
const sent = ref(0)
const gains = ref(0)

const columns = [
  {
    name: "token",
    required: true,
    label: "Tokens",
    field: "token",
  },
  {
    name: "balance",
    required: true,
    label: "Balance",
    sortable: true,
    field: "balance",
  },
  {
    name: "cost",
    required: true,
    label: "Cost",
    sortable: true,
    field: "cost",
  },
  {
    name: "price",
    required: true,
    label: "Price",
    sortable: true,
    field: "price",
  },
  {
    name: "value",
    required: true,
    label: "Value",
    sortable: true,
    field: "value",
  },
]

const wallets = ref<Wallet[]>([])
const icpBalance = ref(0)
const rows = ref<any[]>([
  {
    token: "ICP",
    balance: 0,
    cost: 0,
    price: 0,
    value: 0,
  },
])

onMounted(() => {
  initECharts()
  getWallet()
  getICPPrice()
})

const getBalance = async (address: string, walletName: string) => {
  //获取用户当前钱包资产
  const balance = await getICPBalance(address)
  console.log(address + " balance: ", balance)
  wallets.value.push({
    address: address,
    name: walletName,
    tokens: [{ symbol: "ICP", balance: balance }],
  })
}

watch(
  () => wallets.value.length,
  () => {
    console.log("watch", wallets.value)
    icpBalance.value = wallets.value.reduce(
      // token[0] 目前暂为ICP
      (total, wallet) => total + wallet.tokens[0].balance,
      0,
    )
    rows.value[0].balance = icpBalance.value
    rows.value[0].value = rows.value[0].balance * rows.value[0].price
    console.log("icpBalance", icpBalance.value)
    console.log("rows", rows.value)
  },
)

const getICPPrice = () => {
  getICPNowPrice().then((res) => {
    rows.value[0].price = res
  })
}

const getWallet = async () => {
  const res = await getUserWallet(false)
  if (res.Ok && res.Ok[0]) {
    // let totalHistory: Array<WalletHistory> = []
    for (const walletInfo of res.Ok) {
      //TODO 有bug，多个钱包的资产总值没有计算。
      //将用户的每个钱包地址下的交易记录查出来，并总和到一起
      const walletHistory = await getWalletHistory(walletInfo.address)
      getBalance(walletInfo.address, walletInfo.name)
      totalHistory.value = totalHistory.value.concat(walletHistory.history)
    }
    // 按时间戳排序交易记录数组
    totalHistory.value.sort((a, b) => a.timestamp - b.timestamp)
    console.log("totalHistory", totalHistory.value)
    const timestamps = totalHistory.value.map((record) =>
      new Date(Number(record.timestamp)).toLocaleString(),
    )
    const balances = totalHistory.value.map((record) => record.walletValue)
    getDetail()
    // 基于准备好的dom，初始化echarts实例
    var chart = echarts.init(echartsContainer.value)
    chart.hideLoading()
    // 绘制图表
    chart.setOption({
      title: {
        text: `Total History (${res.Ok.length} wallets)`,
      },
      series: [
        {
          name: "Value",
          type: "line",
          symbol: "none",
          smooth: true,
          sampling: "lttb",
          itemStyle: {
            color: "rgb(255, 70, 131)",
          },
          areaStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              {
                offset: 0,
                color: "rgb(255, 158, 68)",
              },
              {
                offset: 1,
                color: "rgb(255, 70, 131)",
              },
            ]),
          },
          data: balances,
        },
      ],
      xAxis: { data: timestamps },
    })
  } else {
    showMessageError("please add wallet for dashboard")
  }
}
const getDetail = () => {
  if (totalHistory.value) {
    totalHistory.value.forEach((transaction) => {
      if (transaction.type === "RECEIVE") {
        received.value += transaction.amount * transaction.price
      } else if (transaction.type === "SEND") {
        sent.value += transaction.amount * transaction.price
      }
    })
    //保留小数点
    received.value = Number(received.value.toFixed(2))
    sent.value = Number(sent.value.toFixed(2))
    //gain暂且使用这种简单的方式计算
    gains.value = Number((received.value - sent.value).toFixed(2))
  }
}
// 初始化 ECharts 实例
const initECharts = () => {
  const chart = echarts.init(echartsContainer.value)
  // 配置图表选项
  const option = {
    tooltip: {
      trigger: "axis",
      position: function (pt) {
        return [pt[0], "10%"]
      },
    },
    title: {
      left: "center",
      text: "Total History (0 wallets)",
    },
    toolbox: {
      feature: {
        dataZoom: {
          yAxisIndex: "none",
        },
        restore: {},
        saveAsImage: {},
      },
    },
    xAxis: {
      type: "category",
      boundaryGap: false,
      data: [],
    },
    yAxis: {
      type: "value",
      boundaryGap: false, //是否留白
    },
    dataZoom: [
      {
        type: "inside",
        start: 0,
        end: 100,
      },
      {
        //启用图表下方的进度条，方便拖放
        start: 0,
        end: 100,
      },
    ],
  }

  // 使用 setOption 方法将配置应用到图表
  chart.setOption(option)
  chart.showLoading()
}
</script>

<style lang="scss" scoped></style>
