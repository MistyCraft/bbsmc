<template>
  <div>
    <MessageBanner v-if="flags.developerMode" message-type="warning" class="developer-message">
      <CodeIcon class="inline-flex" />
      <IntlFormatted :message-id="developerModeBanner.description">
        <template #strong="{ children }">
          <strong>
            <component :is="() => normalizeChildren(children)" />
          </strong>
        </template>
      </IntlFormatted>
      <Button :action="() => disableDeveloperMode()">
        {{ formatMessage(developerModeBanner.deactivate) }}
      </Button>
    </MessageBanner>
    <section class="universal-card">
      <h2 class="text-2xl">{{ formatMessage(colorTheme.title) }}</h2>
      <p>{{ formatMessage(colorTheme.description) }}</p>
      <div class="theme-options mt-4">
        <button
          v-for="option in themeOptions"
          :key="option"
          class="preview-radio button-base"
          :class="{ selected: theme.preferred === option }"
          @click="() => updateColorTheme(option)"
        >
          <div class="preview" :class="`${option === 'system' ? systemTheme : option}-mode`">
            <div class="example-card card card">
              <div class="example-icon"></div>
              <div class="example-text-1"></div>
              <div class="example-text-2"></div>
            </div>
          </div>
          <div class="label">
            <RadioButtonChecked v-if="theme.preferred === option" class="radio" />
            <RadioButtonIcon v-else class="radio" />
            {{ colorTheme[option] ? formatMessage(colorTheme[option]) : option }}
            <SunIcon
              v-if="theme.preferences.light === option"
              v-tooltip="formatMessage(colorTheme.preferredLight)"
              class="theme-icon"
            />
            <MoonIcon
              v-else-if="theme.preferences.dark === option"
              v-tooltip="formatMessage(colorTheme.preferredDark)"
              class="theme-icon"
            />
          </div>
        </button>
      </div>
    </section>
    <section class="universal-card">
      <h2 class="text-2xl">{{ formatMessage(projectListLayouts.title) }}</h2>
      <p class="mb-4">{{ formatMessage(projectListLayouts.description) }}</p>
      <div class="project-lists">
        <div v-for="projectType in listTypes" :key="projectType.id + '-project-list-layouts'">
          <div class="label">
            <div class="label__title">
              {{
                projectListLayouts[projectType.id]
                  ? formatMessage(projectListLayouts[projectType.id])
                  : projectType.id
              }}
            </div>
          </div>
          <div class="project-list-layouts">
            <button
              class="preview-radio button-base"
              :class="{ selected: cosmetics.searchDisplayMode[projectType.id] === 'list' }"
              @click="() => (cosmetics.searchDisplayMode[projectType.id] = 'list')"
            >
              <div class="preview">
                <div class="layout-list-mode">
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                </div>
              </div>
              <div class="label">
                <RadioButtonChecked
                  v-if="cosmetics.searchDisplayMode[projectType.id] === 'list'"
                  class="radio"
                />
                <RadioButtonIcon v-else class="radio" />
                Rows
              </div>
            </button>
            <button
              class="preview-radio button-base"
              :class="{ selected: cosmetics.searchDisplayMode[projectType.id] === 'grid' }"
              @click="() => (cosmetics.searchDisplayMode[projectType.id] = 'grid')"
            >
              <div class="preview">
                <div class="layout-grid-mode">
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                </div>
              </div>
              <div class="label">
                <RadioButtonChecked
                  v-if="cosmetics.searchDisplayMode[projectType.id] === 'grid'"
                  class="radio"
                />
                <RadioButtonIcon v-else class="radio" />
                Grid
              </div>
            </button>
            <button
              class="preview-radio button-base"
              :class="{ selected: cosmetics.searchDisplayMode[projectType.id] === 'gallery' }"
              @click="() => (cosmetics.searchDisplayMode[projectType.id] = 'gallery')"
            >
              <div class="preview">
                <div class="layout-gallery-mode">
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                  <div class="example-card card"></div>
                </div>
              </div>
              <div class="label">
                <RadioButtonChecked
                  v-if="cosmetics.searchDisplayMode[projectType.id] === 'gallery'"
                  class="radio"
                />
                <RadioButtonIcon v-else class="radio" />
                Gallery
              </div>
            </button>
          </div>
        </div>
      </div>
    </section>
    <section class="universal-card">
      <h2 class="text-2xl">{{ formatMessage(toggleFeatures.title) }}</h2>
      <p class="mb-4">{{ formatMessage(toggleFeatures.description) }}</p>
      <div class="adjacent-input small">
        <label for="advanced-rendering">
          <span class="label__title">
            {{ formatMessage(toggleFeatures.advancedRenderingTitle) }}
          </span>
          <span class="label__description">
            {{ formatMessage(toggleFeatures.advancedRenderingDescription) }}
          </span>
        </label>
        <input
          id="advanced-rendering"
          v-model="cosmetics.advancedRendering"
          class="switch stylized-toggle"
          type="checkbox"
        />
      </div>
      <div class="adjacent-input small">
        <label for="external-links-new-tab">
          <span class="label__title">
            {{ formatMessage(toggleFeatures.externalLinksNewTabTitle) }}
          </span>
          <span class="label__description">
            {{ formatMessage(toggleFeatures.externalLinksNewTabDescription) }}
          </span>
        </label>
        <input
          id="external-links-new-tab"
          v-model="cosmetics.externalLinksNewTab"
          class="switch stylized-toggle"
          type="checkbox"
        />
      </div>
      <div v-if="false" class="adjacent-input small">
        <label for="modrinth-app-promos">
          <span class="label__title">
            {{ formatMessage(toggleFeatures.hideModrinthAppPromosTitle) }}
          </span>
          <span class="label__description">
            {{ formatMessage(toggleFeatures.hideModrinthAppPromosDescription) }}
          </span>
        </label>
        <input
          id="modrinth-app-promos"
          v-model="cosmetics.hideModrinthAppPromos"
          class="switch stylized-toggle"
          type="checkbox"
        />
      </div>
      <div class="adjacent-input small">
        <label for="search-layout-toggle">
          <span class="label__title">
            {{ formatMessage(toggleFeatures.rightAlignedFiltersSidebarTitle) }}
          </span>
          <span class="label__description">
            {{ formatMessage(toggleFeatures.rightAlignedFiltersSidebarDescription) }}
          </span>
        </label>
        <input
          id="search-layout-toggle"
          v-model="cosmetics.rightSearchLayout"
          class="switch stylized-toggle"
          type="checkbox"
        />
      </div>
      <div class="adjacent-input small">
        <label for="project-layout-toggle">
          <span class="label__title">
            {{ formatMessage(toggleFeatures.leftAlignedContentSidebarTitle) }}
          </span>
          <span class="label__description">
            {{ formatMessage(toggleFeatures.leftAlignedContentSidebarDescription) }}
          </span>
        </label>
        <input
          id="project-layout-toggle"
          v-model="cosmetics.leftContentLayout"
          class="switch stylized-toggle"
          type="checkbox"
        />
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { CodeIcon, MoonIcon, RadioButtonChecked, RadioButtonIcon, SunIcon } from "@modrinth/assets";
import { Button } from "@modrinth/ui";
import MessageBanner from "~/components/ui/MessageBanner.vue";
import type { DisplayLocation } from "~/plugins/cosmetics";
import { formatProjectType } from "~/plugins/shorthands.js";
import { isDarkTheme, type Theme } from "~/plugins/theme/index.ts";

useHead({
  title: "外观设置 - BBSMC",
});

const { formatMessage } = useVIntl();

const developerModeBanner = defineMessages({
  description: {
    id: "settings.display.banner.developer-mode.description",
    defaultMessage:
      "<strong>开发者模式</strong> 已启用. This will allow you to view the internal IDs of various things throughout BBSMC that may be helpful if you're a developer using the Modrinth API. Click on the Modrinth logo at the bottom of the page 5 times to toggle developer mode.",
  },
  deactivate: {
    id: "settings.display.banner.developer-mode.button",
    defaultMessage: "停用开发者模式",
  },
});

const colorTheme = defineMessages({
  title: {
    id: "settings.display.theme.title",
    defaultMessage: "主题",
  },
  description: {
    id: "settings.display.theme.description",
    defaultMessage: "在此设备上为 BBSMC 选择您喜欢的颜色主题。",
  },
  system: {
    id: "settings.display.theme.system",
    defaultMessage: "与系统同步",
  },
  light: {
    id: "settings.display.theme.light",
    defaultMessage: "Light",
  },
  dark: {
    id: "settings.display.theme.dark",
    defaultMessage: "Dark",
  },
  oled: {
    id: "settings.display.theme.oled",
    defaultMessage: "黑色",
  },
  retro: {
    id: "settings.display.theme.retro",
    defaultMessage: "Retro",
  },
  preferredLight: {
    id: "settings.display.theme.preferred-light-theme",
    defaultMessage: "Preferred light theme",
  },
  preferredDark: {
    id: "settings.display.theme.preferred-dark-theme",
    defaultMessage: "Preferred dark theme",
  },
});

const projectListLayouts = defineMessages({
  title: {
    id: "settings.display.project-list-layouts.title",
    defaultMessage: "Project list layouts",
  },
  description: {
    id: "settings.display.project-list-layouts.description",
    defaultMessage:
      "Select your preferred layout for each page that displays project lists on this device.",
  },
  mod: {
    id: "settings.display.project-list-layouts.mod",
    defaultMessage: "Mods page",
  },
  plugin: {
    id: "settings.display.project-list-layouts.plugin",
    defaultMessage: "Plugins page",
  },
  datapack: {
    id: "settings.display.project-list-layouts.datapack",
    defaultMessage: "Data Packs page",
  },
  shader: {
    id: "settings.display.project-list-layouts.shader",
    defaultMessage: "Shaders page",
  },
  resourcepack: {
    id: "settings.display.project-list-layouts.resourcepack",
    defaultMessage: "Resource Packs page",
  },
  modpack: {
    id: "settings.display.project-list-layouts.modpack",
    defaultMessage: "Modpacks page",
  },
  user: {
    id: "settings.display.project-list-layouts.user",
    defaultMessage: "User profile pages",
  },
  collection: {
    id: "settings.display.project-list.layouts.collection",
    defaultMessage: "Collection",
  },
});

const toggleFeatures = defineMessages({
  title: {
    id: "settings.display.flags.title",
    defaultMessage: "Toggle features",
  },
  description: {
    id: "settings.display.flags.description",
    defaultMessage: "Enable or disable certain features on this device.",
  },
  advancedRenderingTitle: {
    id: "settings.display.sidebar.advanced-rendering.title",
    defaultMessage: "Advanced rendering",
  },
  advancedRenderingDescription: {
    id: "settings.display.sidebar.advanced-rendering.description",
    defaultMessage:
      "Enables advanced rendering such as blur effects that may cause performance issues without hardware-accelerated rendering.",
  },
  externalLinksNewTabTitle: {
    id: "settings.display.sidebar.external-links-new-tab.title",
    defaultMessage: "Open external links in new tab",
  },
  externalLinksNewTabDescription: {
    id: "settings.display.sidebar.external-links-new-tab.description",
    defaultMessage:
      "Make links which go outside of Modrinth open in a new tab. No matter this setting, links on the same domain and in Markdown descriptions will open in the same tab, and links on ads and edit pages will open in a new tab.",
  },
  hideModrinthAppPromosTitle: {
    id: "settings.display.sidebar.hide-app-promos.title",
    defaultMessage: "Hide Modrinth App promotions",
  },
  hideModrinthAppPromosDescription: {
    id: "settings.display.sidebar.hide-app-promos.description",
    defaultMessage:
      'Hides the "Get Modrinth App" buttons from primary navigation. The Modrinth App page can still be found on the landing page or in the footer.',
  },
  rightAlignedFiltersSidebarTitle: {
    id: "settings.display.sidebar.right-aligned-filters-sidebar.title",
    defaultMessage: "搜索页面上右对齐的过滤器侧栏s",
  },
  rightAlignedFiltersSidebarDescription: {
    id: "settings.display.sidebar.right-aligned-filters-sidebar.description",
    defaultMessage: "将过滤器侧边栏与搜索结果右侧对齐。",
  },
  leftAlignedContentSidebarTitle: {
    id: "settings.display.sidebar.left-aligned-content-sidebar.title",
    defaultMessage: "内容页左对齐侧边栏",
  },
  leftAlignedContentSidebarDescription: {
    id: "settings.display.sidebar.right-aligned-content-sidebar.description",
    defaultMessage: "将侧边栏与页面内容左侧对齐.",
  },
});

const cosmetics = useCosmetics();
const flags = useFeatureFlags();
const tags = useTags();

const theme = useTheme();

// On the server the value of native theme can be 'unknown'. To hydrate
// correctly, we need to make sure we aren't using 'unknown' and values between
// server and client renders are in sync.

const serverSystemTheme = useState(() => {
  const theme_ = theme.native;
  if (theme_ === "unknown") return "oled";
  return theme_;
});

const systemTheme = useMountedValue((mounted): Theme => {
  const systemTheme_ = mounted ? theme.native : serverSystemTheme.value;
  return systemTheme_ === "light" ? theme.preferences.light : theme.preferences.dark;
});

const themeOptions = computed(() => {
  const options: ("system" | Theme)[] = ["dark", "light"];
  // const options: ("system" | Theme)[] = ["system", "light", "dark", "oled"];
  if (flags.value.developerMode || theme.preferred === "retro") {
    options.push("retro");
  }
  return options;
});

function updateColorTheme(value: Theme | "system") {
  if (value !== "system") {
    if (isDarkTheme(value)) {
      theme.preferences.dark = value;
    } else {
      theme.preferences.light = value;
    }
  }

  theme.preferred = value;
}

function disableDeveloperMode() {
  flags.value.developerMode = !flags.value.developerMode;
  saveFeatureFlags();
  addNotification({
    group: "main",
    title: "开发者模式已停用",
    text: "Developer mode has been disabled",
    type: "success",
  });
}

const listTypes = computed(() => {
  const types = tags.value.projectTypes.map((type) => {
    return {
      id: type.id as DisplayLocation,
      name: formatProjectType(type.id) + "s",
      display: "the " + formatProjectType(type.id).toLowerCase() + "s search page",
    };
  });

  types.push({
    id: "user" as DisplayLocation,
    name: "User profiles",
    display: "user pages",
  });

  return types;
});
</script>
<style scoped lang="scss">
.preview-radio {
  width: 100%;
  border-radius: var(--radius-md);
  padding: 0;
  overflow: hidden;
  border: 1px solid var(--color-divider);
  background-color: var(--color-button-bg);
  color: var(--color-base);
  display: flex;
  flex-direction: column;
  outline: 2px solid transparent;

  &.selected {
    color: var(--color-contrast);

    .label {
      .radio {
        color: var(--color-brand);
      }

      .theme-icon {
        color: var(--color-text);
      }
    }
  }

  .preview {
    background-color: var(--color-bg);
    padding: 1.5rem;
    outline: 2px solid transparent;
    width: 100%;

    .example-card {
      margin: 0;
      padding: 1rem;
      outline: 2px solid transparent;
      min-height: 0;
    }
  }

  .label {
    display: flex;
    align-items: center;
    text-align: left;
    flex-grow: 1;
    padding: var(--gap-md) var(--gap-lg);

    .radio {
      margin-right: 0.5rem;
    }

    .theme-icon {
      color: var(--color-secondary);
      margin-left: 0.25rem;
    }
  }
}

.theme-options {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(12.5rem, 1fr));
  gap: var(--gap-lg);

  .preview .example-card {
    margin: 0;
    padding: 1rem;
    display: grid;
    grid-template: "icon text1" "icon text2";
    grid-template-columns: auto 1fr;
    gap: 0.5rem;
    outline: 2px solid transparent;

    .example-icon {
      grid-area: icon;
      width: 2rem;
      height: 2rem;
      background-color: var(--color-button-bg);
      border-radius: var(--radius-sm);
      outline: 2px solid transparent;
    }

    .example-text-1,
    .example-text-2 {
      height: 0.5rem;
      border-radius: var(--radius-sm);
      outline: 2px solid transparent;
    }

    .example-text-1 {
      grid-area: text1;
      width: 100%;
      background-color: var(--color-base);
    }

    .example-text-2 {
      grid-area: text2;
      width: 60%;
      background-color: var(--color-secondary);
    }
  }
}

.project-lists {
  display: flex;
  flex-direction: column;
  gap: var(--gap-md);

  > :first-child .label__title {
    margin-top: 0;
  }

  .preview {
    --_layout-width: 7rem;
    --_layout-height: 4.5rem;
    --_layout-gap: 0.25rem;

    .example-card {
      border-radius: 0.5rem;
      width: var(--_layout-width);
      height: calc((var(--_layout-height) - 3 * var(--_layout-gap)) / 4);
      padding: 0;
    }

    .layout-list-mode {
      display: grid;
      grid-template-columns: 1fr;
      gap: var(--_layout-gap);
    }

    .layout-grid-mode {
      display: grid;
      grid-template-columns: 1fr 1fr 1fr;
      gap: var(--_layout-gap);

      .example-card {
        width: calc((var(--_layout-width) - 2 * var(--_layout-gap)) / 3);
        height: calc((var(--_layout-height) - var(--_layout-gap)) / 2);
      }
    }

    .layout-gallery-mode {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: var(--_layout-gap);

      .example-card {
        width: calc((var(--_layout-width) - var(--_layout-gap)) / 2);
        height: calc((var(--_layout-height) - var(--_layout-gap)) / 2);
      }
    }
  }
}

.project-list-layouts {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(9.5rem, 1fr));
  gap: var(--gap-lg);

  .preview-radio .example-card {
    border: 2px solid transparent;
  }

  .preview-radio.selected .example-card {
    border-color: var(--color-brand);
    background-color: var(--color-brand-highlight);
  }

  .preview {
    display: flex;
    align-items: center;
    justify-content: center;
  }
}

.developer-message {
  svg {
    vertical-align: middle;
    margin-bottom: 2px;
    margin-right: 0.5rem;
  }

  .btn {
    margin-top: var(--gap-sm);
  }
}
</style>
