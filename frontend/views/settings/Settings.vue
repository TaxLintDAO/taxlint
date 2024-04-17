<template>
  <div class="settings-container">
    <div class="row">
      <div class="col-md-12 col-lg-8">
        <q-card class="my-card">
          <q-card-section>
            <div class="text-h6">Portfolio settings</div>
          </q-card-section>

          <q-card-section class="q-pt-none">
            <q-form @submit="onSubmit" ref="form" class="q-gutter-md">
              <q-list>
                <q-item>
                  <q-item-section>
                    <q-item-label> Base currency </q-item-label>
                    <q-item-label caption>
                      <q-select
                        v-model="currencyModel"
                        filled
                        @update:model-value="getPrice()"
                        option-label="code"
                        option-value="code"
                        :options="currencys"
                        label="Select Base Currency *"
                        :rules="[
                          (val) => !!val || 'Please select base currency',
                        ]"
                      >
                        <template v-slot:option="scope">
                          <q-item v-bind="scope.itemProps">
                            <q-item-section>
                              <q-item-label>{{ scope.opt.code }}</q-item-label>
                              <q-item-label caption>{{
                                scope.opt.name
                              }}</q-item-label>
                            </q-item-section>
                          </q-item>
                        </template></q-select
                      >
                    </q-item-label>
                  </q-item-section>
                </q-item>
                <q-item>
                  <q-item-section>
                    <q-item-label> Timezone for reports </q-item-label>
                    <q-item-label caption>
                      <q-select
                        v-model="currencyModel"
                        filled
                        :options="currencys"
                        label="Select Timezone *"
                        :rules="[(val) => !!val || 'Please select timezone']"
                      ></q-select>
                    </q-item-label>
                  </q-item-section>
                </q-item>
                <q-item>
                  <q-item-section>
                    <q-item-label> Cost basis method </q-item-label>
                    <q-item-label caption>
                      <q-select
                        v-model="costMethod"
                        filled
                        :options="costMethodOption"
                        label="Select cost basis method *"
                        :rules="[
                          (val) => !!val || 'Please select cost basis method',
                        ]"
                      ></q-select>
                    </q-item-label>
                  </q-item-section>
                </q-item>
              </q-list>

              <div class="q-gutter-sm justify-end flex">
                <q-btn flat label="Cancel" v-close-popup="true" />
                <q-btn
                  :loading="loading"
                  label="Save"
                  type="submit"
                  color="primary"
                />
              </div>
            </q-form>
          </q-card-section>
        </q-card>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getBaseCurrencyPrice } from "@/api/baseCurrencies"
import baseCurrencies from "@/utils/currencies"
import type { QForm } from "quasar"
import { onMounted, ref } from "vue"

const form = ref<QForm | null>(null)
const loading = ref(false)
const currencys = baseCurrencies
const currencyModel = ref()
const costMethod = ref("FIFO")
const costMethodOption = ["FIFO", "LIFO", "HIFO"]

onMounted(() => {})
const getPrice = () => {
  getBaseCurrencyPrice(currencyModel.value.code)
}

const onSubmit = async () => {
  loading.value = true
  const validationSuccess = await form.value?.validate()

  try {
    if (validationSuccess) {
    } else {
      // 数据验证失败
      // 用户至少输入了一个无效值
    }
  } catch (error) {
  } finally {
    loading.value = false
  }
}
</script>

<style lang="scss">
.settings-container {
}
</style>
@/utils/currencies