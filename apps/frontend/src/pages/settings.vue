<template>
  <div>
    <div class="normal-page no-sidebar">
      <h1>{{ formatMessage(commonMessages.settingsLabel) }}</h1>
    </div>
    <div class="normal-page">
      <div class="normal-page__sidebar">
        <aside class="universal-card">
          <NavStack>
            <h3>显示</h3>
            <NavStackItem
              link="/settings"
              :label="formatMessage(commonSettingsMessages.appearance)"
            >
              <PaintBrushIcon />
            </NavStackItem>
            <!--            <NavStackItem-->
            <!--              v-if="isStaging"-->
            <!--              link="/settings/language"-->
            <!--              :label="formatMessage(commonSettingsMessages.language)"-->
            <!--            >-->
            <!--              <LanguagesIcon />-->
            <!--            </NavStackItem>-->
            <template v-if="auth.user">
              <h3>账户</h3>
              <NavStackItem
                link="/settings/profile"
                :label="formatMessage(commonSettingsMessages.profile)"
              >
                <UserIcon />
              </NavStackItem>
              <NavStackItem
                link="/settings/account"
                :label="formatMessage(commonSettingsMessages.account)"
              >
                <ShieldIcon />
              </NavStackItem>
              <!--              <NavStackItem-->
              <!--                link="/settings/authorizations"-->
              <!--                :label="formatMessage(commonSettingsMessages.authorizedApps)"-->
              <!--              >-->
              <!--                <GridIcon />-->
              <!--              </NavStackItem>-->
              <NavStackItem
                link="/settings/sessions"
                :label="formatMessage(commonSettingsMessages.sessions)"
              >
                <MonitorSmartphoneIcon />
              </NavStackItem>
              <!--              <NavStackItem-->
              <!--                link="/settings/billing"-->
              <!--                :label="formatMessage(commonSettingsMessages.billing)"-->
              <!--              >-->
              <!--                <CardIcon />-->
              <!--              </NavStackItem>-->
            </template>
            <!--            <template v-if="auth.user">-->
            <!--              <h3>开发</h3>-->
            <!--              <NavStackItem-->
            <!--                link="/settings/pats"-->
            <!--                :label="formatMessage(commonSettingsMessages.pats)"-->
            <!--              >-->
            <!--                <KeyIcon />-->
            <!--              </NavStackItem>-->
            <!--              <NavStackItem-->
            <!--                link="/settings/applications"-->
            <!--                :label="formatMessage(commonSettingsMessages.applications)"-->
            <!--              >-->
            <!--                <ServerIcon />-->
            <!--              </NavStackItem>-->
            <!--            </template>-->
          </NavStack>
        </aside>
      </div>
      <div class="normal-page__content">
        <NuxtPage :route="route" />
      </div>
    </div>
  </div>
</template>
<script setup>
import { PaintBrushIcon, UserIcon, ShieldIcon } from "@modrinth/assets";

import NavStack from "~/components/ui/NavStack.vue";
import NavStackItem from "~/components/ui/NavStackItem.vue";
import MonitorSmartphoneIcon from "~/assets/images/utils/monitor-smartphone.svg?component";

import { commonMessages, commonSettingsMessages } from "~/utils/common-messages.ts";

const { formatMessage } = useVIntl();

const route = useNativeRoute();
const auth = await useAuth();
</script>
