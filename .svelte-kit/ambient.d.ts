
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const SHELL: string;
	export const WINDOWID: string;
	export const __ETC_PROFILE_DONE: string;
	export const COLORTERM: string;
	export const __HM_SESS_VARS_SOURCED: string;
	export const XDG_CONFIG_DIRS: string;
	export const XDG_SESSION_PATH: string;
	export const NIX_BUILD_CORES: string;
	export const TERM_PROGRAM_VERSION: string;
	export const GTK_IM_MODULE: string;
	export const configureFlags: string;
	export const SPEECHD_CMD: string;
	export const mesonFlags: string;
	export const TMUX: string;
	export const PKG_CONFIG_PATH: string;
	export const FORGIT_INSTALL_DIR: string;
	export const shell: string;
	export const depsHostHost: string;
	export const NODE: string;
	export const TAURI_ENV_DEBUG: string;
	export const LC_ADDRESS: string;
	export const LC_NAME: string;
	export const SSH_AUTH_SOCK: string;
	export const STRINGS: string;
	export const depsTargetTarget: string;
	export const TPM2_PKCS11_TCTI: string;
	export const XCURSOR_PATH: string;
	export const stdenv: string;
	export const LOCALE_ARCHIVE_2_27: string;
	export const npm_config_local_prefix: string;
	export const builder: string;
	export const XMODIFIERS: string;
	export const DESKTOP_SESSION: string;
	export const LC_MONETARY: string;
	export const GDK_PIXBUF_MODULE_FILE: string;
	export const NO_AT_BRIDGE: string;
	export const shellHook: string;
	export const XCURSOR_SIZE: string;
	export const GPG_TTY: string;
	export const EDITOR: string;
	export const phases: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const XDG_SEAT: string;
	export const PWD: string;
	export const NIX_PROFILES: string;
	export const SOURCE_DATE_EPOCH: string;
	export const LOGNAME: string;
	export const XDG_SESSION_DESKTOP: string;
	export const QT_QPA_PLATFORMTHEME: string;
	export const TAURI_ENV_PLATFORM: string;
	export const XDG_SESSION_TYPE: string;
	export const NIX_ENFORCE_NO_NATIVE: string;
	export const CUPS_DATADIR: string;
	export const NIX_PATH: string;
	export const NIXPKGS_CONFIG: string;
	export const CXX: string;
	export const TPM2TOOLS_TCTI: string;
	export const _: string;
	export const XAUTHORITY: string;
	export const TEMPDIR: string;
	export const system: string;
	export const XDG_GREETER_DATA_DIR: string;
	export const HOST_PATH: string;
	export const QT_STYLE_OVERRIDE: string;
	export const GTK2_RC_FILES: string;
	export const IN_NIX_SHELL: string;
	export const doInstallCheck: string;
	export const HOME: string;
	export const NIX_BINTOOLS: string;
	export const GETTEXTDATADIRS: string;
	export const SSH_ASKPASS: string;
	export const LANG: string;
	export const LC_PAPER: string;
	export const TMUX_TMPDIR: string;
	export const LS_COLORS: string;
	export const _JAVA_AWT_WM_NONREPARENTING: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const depsTargetTargetPropagated: string;
	export const npm_package_version: string;
	export const cmakeFlags: string;
	export const WEBKIT_DISABLE_DMABUF_RENDERER: string;
	export const GIO_EXTRA_MODULES: string;
	export const outputs: string;
	export const NIX_STORE: string;
	export const TMPDIR: string;
	export const XDG_SEAT_PATH: string;
	export const LD: string;
	export const buildPhase: string;
	export const DIRENV_CONFIG: string;
	export const dbus: string;
	export const READELF: string;
	export const GTK_A11Y: string;
	export const FORGIT: string;
	export const NIX_USER_PROFILE_DIR: string;
	export const INFOPATH: string;
	export const doCheck: string;
	export const TAURI_ENV_PLATFORM_VERSION: string;
	export const depsBuildBuild: string;
	export const XDG_SESSION_CLASS: string;
	export const TAURI_ENV_FAMILY: string;
	export const LC_IDENTIFICATION: string;
	export const TERM: string;
	export const npm_package_name: string;
	export const GTK_PATH: string;
	export const SIZE: string;
	export const propagatedNativeBuildInputs: string;
	export const LESSOPEN: string;
	export const USER: string;
	export const strictDeps: string;
	export const TMUX_PANE: string;
	export const TZDIR: string;
	export const AR: string;
	export const AS: string;
	export const TAURI_ENV_TARGET_TRIPLE: string;
	export const TEMP: string;
	export const DISPLAY: string;
	export const NIX_BINTOOLS_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
	export const npm_lifecycle_event: string;
	export const SHLVL: string;
	export const NIX_BUILD_TOP: string;
	export const NM: string;
	export const PAGER: string;
	export const NIX_CFLAGS_COMPILE: string;
	export const LC_TELEPHONE: string;
	export const QTWEBKIT_PLUGIN_PATH: string;
	export const patches: string;
	export const QT_IM_MODULE: string;
	export const TAURI_CLI_VERBOSITY: string;
	export const LC_MEASUREMENT: string;
	export const __NIXOS_SET_ENVIRONMENT_DONE: string;
	export const XDG_VTNR: string;
	export const buildInputs: string;
	export const XDG_SESSION_ID: string;
	export const LOCALE_ARCHIVE: string;
	export const preferLocalBuild: string;
	export const LESSKEYIN_SYSTEM: string;
	export const npm_config_user_agent: string;
	export const QML2_IMPORT_PATH: string;
	export const TERMINFO_DIRS: string;
	export const GSETTINGS_SCHEMAS_PATH: string;
	export const npm_execpath: string;
	export const XDG_RUNTIME_DIR: string;
	export const NIX_PKG_CONFIG_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
	export const depsBuildTarget: string;
	export const OBJCOPY: string;
	export const NIX_XDG_DESKTOP_PORTAL_DIR: string;
	export const out: string;
	export const LC_TIME: string;
	export const npm_package_json: string;
	export const TAURI_ENV_ARCH: string;
	export const STRIP: string;
	export const JOURNAL_STREAM: string;
	export const XCURSOR_THEME: string;
	export const XDG_DATA_DIRS: string;
	export const LIBEXEC_PATH: string;
	export const TMP: string;
	export const OBJDUMP: string;
	export const PATH: string;
	export const propagatedBuildInputs: string;
	export const __fish_nixos_env_preinit_sourced: string;
	export const CC: string;
	export const GDMSESSION: string;
	export const NIX_CC: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const depsBuildTargetPropagated: string;
	export const depsBuildBuildPropagated: string;
	export const NIX_CC_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
	export const QT_PLUGIN_PATH: string;
	export const CONFIG_SHELL: string;
	export const __structuredAttrs: string;
	export const npm_node_execpath: string;
	export const RANLIB: string;
	export const NIX_HARDENING_ENABLE: string;
	export const LC_NUMERIC: string;
	export const OLDPWD: string;
	export const NIX_LDFLAGS: string;
	export const nativeBuildInputs: string;
	export const name: string;
	export const PKG_CONFIG: string;
	export const TERM_PROGRAM: string;
	export const depsHostHostPropagated: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		SHELL: string;
		WINDOWID: string;
		__ETC_PROFILE_DONE: string;
		COLORTERM: string;
		__HM_SESS_VARS_SOURCED: string;
		XDG_CONFIG_DIRS: string;
		XDG_SESSION_PATH: string;
		NIX_BUILD_CORES: string;
		TERM_PROGRAM_VERSION: string;
		GTK_IM_MODULE: string;
		configureFlags: string;
		SPEECHD_CMD: string;
		mesonFlags: string;
		TMUX: string;
		PKG_CONFIG_PATH: string;
		FORGIT_INSTALL_DIR: string;
		shell: string;
		depsHostHost: string;
		NODE: string;
		TAURI_ENV_DEBUG: string;
		LC_ADDRESS: string;
		LC_NAME: string;
		SSH_AUTH_SOCK: string;
		STRINGS: string;
		depsTargetTarget: string;
		TPM2_PKCS11_TCTI: string;
		XCURSOR_PATH: string;
		stdenv: string;
		LOCALE_ARCHIVE_2_27: string;
		npm_config_local_prefix: string;
		builder: string;
		XMODIFIERS: string;
		DESKTOP_SESSION: string;
		LC_MONETARY: string;
		GDK_PIXBUF_MODULE_FILE: string;
		NO_AT_BRIDGE: string;
		shellHook: string;
		XCURSOR_SIZE: string;
		GPG_TTY: string;
		EDITOR: string;
		phases: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		XDG_SEAT: string;
		PWD: string;
		NIX_PROFILES: string;
		SOURCE_DATE_EPOCH: string;
		LOGNAME: string;
		XDG_SESSION_DESKTOP: string;
		QT_QPA_PLATFORMTHEME: string;
		TAURI_ENV_PLATFORM: string;
		XDG_SESSION_TYPE: string;
		NIX_ENFORCE_NO_NATIVE: string;
		CUPS_DATADIR: string;
		NIX_PATH: string;
		NIXPKGS_CONFIG: string;
		CXX: string;
		TPM2TOOLS_TCTI: string;
		_: string;
		XAUTHORITY: string;
		TEMPDIR: string;
		system: string;
		XDG_GREETER_DATA_DIR: string;
		HOST_PATH: string;
		QT_STYLE_OVERRIDE: string;
		GTK2_RC_FILES: string;
		IN_NIX_SHELL: string;
		doInstallCheck: string;
		HOME: string;
		NIX_BINTOOLS: string;
		GETTEXTDATADIRS: string;
		SSH_ASKPASS: string;
		LANG: string;
		LC_PAPER: string;
		TMUX_TMPDIR: string;
		LS_COLORS: string;
		_JAVA_AWT_WM_NONREPARENTING: string;
		XDG_CURRENT_DESKTOP: string;
		depsTargetTargetPropagated: string;
		npm_package_version: string;
		cmakeFlags: string;
		WEBKIT_DISABLE_DMABUF_RENDERER: string;
		GIO_EXTRA_MODULES: string;
		outputs: string;
		NIX_STORE: string;
		TMPDIR: string;
		XDG_SEAT_PATH: string;
		LD: string;
		buildPhase: string;
		DIRENV_CONFIG: string;
		dbus: string;
		READELF: string;
		GTK_A11Y: string;
		FORGIT: string;
		NIX_USER_PROFILE_DIR: string;
		INFOPATH: string;
		doCheck: string;
		TAURI_ENV_PLATFORM_VERSION: string;
		depsBuildBuild: string;
		XDG_SESSION_CLASS: string;
		TAURI_ENV_FAMILY: string;
		LC_IDENTIFICATION: string;
		TERM: string;
		npm_package_name: string;
		GTK_PATH: string;
		SIZE: string;
		propagatedNativeBuildInputs: string;
		LESSOPEN: string;
		USER: string;
		strictDeps: string;
		TMUX_PANE: string;
		TZDIR: string;
		AR: string;
		AS: string;
		TAURI_ENV_TARGET_TRIPLE: string;
		TEMP: string;
		DISPLAY: string;
		NIX_BINTOOLS_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
		npm_lifecycle_event: string;
		SHLVL: string;
		NIX_BUILD_TOP: string;
		NM: string;
		PAGER: string;
		NIX_CFLAGS_COMPILE: string;
		LC_TELEPHONE: string;
		QTWEBKIT_PLUGIN_PATH: string;
		patches: string;
		QT_IM_MODULE: string;
		TAURI_CLI_VERBOSITY: string;
		LC_MEASUREMENT: string;
		__NIXOS_SET_ENVIRONMENT_DONE: string;
		XDG_VTNR: string;
		buildInputs: string;
		XDG_SESSION_ID: string;
		LOCALE_ARCHIVE: string;
		preferLocalBuild: string;
		LESSKEYIN_SYSTEM: string;
		npm_config_user_agent: string;
		QML2_IMPORT_PATH: string;
		TERMINFO_DIRS: string;
		GSETTINGS_SCHEMAS_PATH: string;
		npm_execpath: string;
		XDG_RUNTIME_DIR: string;
		NIX_PKG_CONFIG_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
		depsBuildTarget: string;
		OBJCOPY: string;
		NIX_XDG_DESKTOP_PORTAL_DIR: string;
		out: string;
		LC_TIME: string;
		npm_package_json: string;
		TAURI_ENV_ARCH: string;
		STRIP: string;
		JOURNAL_STREAM: string;
		XCURSOR_THEME: string;
		XDG_DATA_DIRS: string;
		LIBEXEC_PATH: string;
		TMP: string;
		OBJDUMP: string;
		PATH: string;
		propagatedBuildInputs: string;
		__fish_nixos_env_preinit_sourced: string;
		CC: string;
		GDMSESSION: string;
		NIX_CC: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		depsBuildTargetPropagated: string;
		depsBuildBuildPropagated: string;
		NIX_CC_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
		QT_PLUGIN_PATH: string;
		CONFIG_SHELL: string;
		__structuredAttrs: string;
		npm_node_execpath: string;
		RANLIB: string;
		NIX_HARDENING_ENABLE: string;
		LC_NUMERIC: string;
		OLDPWD: string;
		NIX_LDFLAGS: string;
		nativeBuildInputs: string;
		name: string;
		PKG_CONFIG: string;
		TERM_PROGRAM: string;
		depsHostHostPropagated: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
