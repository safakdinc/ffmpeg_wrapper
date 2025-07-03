export const useTauri = () => {
  const isTauri = ref(false);

  // Check if we're running in Tauri environment
  onMounted(() => {
    if (typeof window !== 'undefined' && window.__TAURI__) {
      isTauri.value = true;
    }
  });

  // Window management
  const windowControls = {
    async minimize() {
      try {
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        await getCurrentWindow().minimize();
        return true;
      } catch (error) {
        console.error('Error minimizing window:', error);
        return false;
      }
    },

    async maximize() {
      try {
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        await getCurrentWindow().maximize();
        return true;
      } catch (error) {
        console.error('Error maximizing window:', error);
        return false;
      }
    },

    async unmaximize() {
      try {
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        await getCurrentWindow().unmaximize();
        return true;
      } catch (error) {
        console.error('Error unmaximizing window:', error);
        return false;
      }
    },

    async toggleMaximize() {
      try {
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        const window = getCurrentWindow();
        const isMaximized = await window.isMaximized();

        if (isMaximized) {
          await window.unmaximize();
        } else {
          await window.maximize();
        }
        return true;
      } catch (error) {
        console.error('Error toggling maximize:', error);
        return false;
      }
    },

    async close() {
      try {
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        await getCurrentWindow().close();
        return true;
      } catch (error) {
        console.error('Error closing window:', error);
        return false;
      }
    }
  };

  // Dialog utilities
  const dialog = {
    async showMessage(message: string, title?: string) {
      try {
        const { message: showMessage } = await import('@tauri-apps/plugin-dialog');
        await showMessage(message, title);
        return true;
      } catch (error) {
        console.error('Error showing message:', error);
        return false;
      }
    },

    async showConfirm(message: string, title?: string) {
      try {
        const { confirm } = await import('@tauri-apps/plugin-dialog');
        return await confirm(message, title);
      } catch (error) {
        console.error('Error showing confirm dialog:', error);
        return false;
      }
    },

    async showError(message: string, title?: string) {
      try {
        const { message: showMessage } = await import('@tauri-apps/plugin-dialog');
        await showMessage(message, title || 'Error');
        return true;
      } catch (error) {
        console.error('Error showing error dialog:', error);
        return false;
      }
    },

    async open(options: { title?: string; multiple?: boolean; filters?: Array<{ name: string; extensions: string[] }> }) {
      try {
        const { open } = await import('@tauri-apps/plugin-dialog');
        return await open(options);
      } catch (error) {
        console.error('Error opening file dialog:', error);
        return null;
      }
    },

    async save(options: { title?: string; defaultPath?: string; filters?: Array<{ name: string; extensions: string[] }> }) {
      try {
        const { save } = await import('@tauri-apps/plugin-dialog');
        return await save(options);
      } catch (error) {
        console.error('Error opening save dialog:', error);
        return null;
      }
    }
  };

  // Notification utilities
  const notification = {
    async send(title: string, body: string) {
      try {
        const { sendNotification } = await import('@tauri-apps/plugin-notification');
        await sendNotification({ title, body });
        return true;
      } catch (error) {
        console.error('Error sending notification:', error);
        return false;
      }
    }
  };

  // Platform information
  const platform = {
    async getInfo() {
      try {
        const { platform, arch, version } = await import('@tauri-apps/plugin-os');
        return {
          platform: await platform(),
          arch: await arch(),
          version: await version()
        };
      } catch (error) {
        console.error('Error getting platform info:', error);
        return null;
      }
    }
  };

  // Tauri invoke function
  const invoke = async (command: string, args?: Record<string, unknown>) => {
    try {
      const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
      return await tauriInvoke(command, args);
    } catch (error) {
      console.error(`Error invoking command ${command}:`, error);
      throw error;
    }
  };

  // Event listener function
  const listen = async (event: string, callback: (event: { payload: unknown }) => void) => {
    try {
      const { listen: tauriListen } = await import('@tauri-apps/api/event');
      return await tauriListen(event, callback);
    } catch (error) {
      console.error(`Error listening to event ${event}:`, error);
      return () => {};
    }
  };

  return {
    isTauri: readonly(isTauri),
    windowControls,
    dialog,
    notification,
    platform,
    invoke,
    listen
  };
};
