import React from "react";
import type { AppId } from "@/lib/api/types";
import { ClaudeIcon, CodexIcon, GeminiIcon } from "@/components/BrandIcons";
import { ProviderIcon } from "@/components/ProviderIcon";

export interface AppConfig {
  label: string;
  icon: React.ReactNode;
  activeClass: string;
  badgeClass: string;
}

export const APP_IDS: AppId[] = [
  "claude",
  "codex",
  "gemini",
  "opencode",
  "antigravity",
];

export const APP_ICON_MAP: Record<AppId, AppConfig> = {
  // ...
  antigravity: {
    label: "Antigravity",
    icon: (
      <ProviderIcon
        icon="antigravity"
        name="Antigravity"
        size={14}
        showFallback={false}
      />
    ),
    activeClass:
      "bg-purple-500/10 ring-1 ring-purple-500/20 hover:bg-purple-500/20 text-purple-600 dark:text-purple-400",
    badgeClass:
      "bg-purple-500/10 text-purple-700 dark:text-purple-300 hover:bg-purple-500/20 border-0 gap-1.5",
  },
};
