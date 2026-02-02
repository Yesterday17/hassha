import type { Plugin } from "@opencode-ai/plugin"
import { join } from "node:path"
import { homedir } from "node:os"

// Binary is installed to ~/.config/opencode/bin/hassha by `hassha install --open-code`
const HASSHA_BINARY = join(homedir(), ".config", "opencode", "bin", "hassha")

/**
 * Hassha Plugin for OpenCode
 *
 * Plays JR East departure melodies on various OpenCode events.
 * Configure melodies per project using .hassha/config.toml
 *
 * Event Mapping (OpenCode → Claude Code equivalent):
 * - session.created    → SessionStart
 * - session.deleted    → SessionEnd
 * - session.idle       → Stop
 * - session.error      → Notification
 * - session.compacted  → PreCompact
 * - permission.asked   → PermissionRequest
 * - tool.execute.before → PreToolUse
 * - tool.execute.after  → PostToolUse
 */
export const plugin: Plugin = async ({ $, directory }) => {
  // Helper to run hassha hook command
  const runHook = async (event: string, payload?: object) => {
    try {
      const input = JSON.stringify({
        hook_event_name: event,
        cwd: directory,
        ...payload,
      })
      await $`echo ${input} | ${HASSHA_BINARY} hook ${event}`.quiet()
    } catch(e) {
      console.error("Error running hassha hook:", e)
    }
  }

  return {
    // Subscribe to OpenCode events
    event: async ({ event }) => {
      switch (event.type) {
        // === Session Events ===
        case "session.created":
          // New session started
          await runHook("SessionStart", { source: "startup" })
          break

        case "session.deleted":
          // Session terminated
          await runHook("SessionEnd", { reason: "other" })
          break

        case "session.idle":
          // Session finished responding - equivalent to Claude Code's "Stop"
          await runHook("Stop")
          break

        case "session.error":
          // An error occurred
          await runHook("Notification", { notification_type: "error" })
          break

        case "session.compacted":
          // Session was compacted (after the fact in OpenCode)
          await runHook("PreCompact", { trigger: "auto" })
          break

        // === Permission Events ===
        // Note: OpenCode does not have a direct equivalent for permission request sounds
        // case "permission.asked":
        //   // Permission dialog appeared
        //   await runHook("PermissionRequest", {
        //     tool_name:
        //       (event as { properties?: { tool?: string } }).properties?.tool ||
        //       "unknown",
        //   })
        //   await runHook("Notification", {
        //     notification_type: "permission_prompt",
        //   })
        //   break

        case "permission.replied":
          // User responded to permission - could trigger a post-permission sound
          // No direct Claude Code equivalent
          break
      }
    },

    // Hook into tool execution - before
    "tool.execute.before": async (input, _output) => {
      await runHook("PreToolUse", {
        tool_name: input.tool,
        // tool_input: input.args,
      })
    },

    // Hook into tool execution - after
    "tool.execute.after": async (input, output) => {
      // Check if the tool execution succeeded or failed
      const hasError =
        output &&
        typeof output === "object" &&
        ("error" in output || "isError" in output)

      if (hasError) {
        await runHook("PostToolUseFailure", {
          tool_name: input.tool,
          // tool_input: input.args,
        })
      } else {
        await runHook("PostToolUse", {
          tool_name: input.tool,
          // tool_input: input.args,
        })
      }
    },
  }
}

export default plugin
