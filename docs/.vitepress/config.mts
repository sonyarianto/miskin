import { defineConfig } from 'vitepress'

export default defineConfig({
  lang: 'en-US',
  title: 'Miskin',
  description: 'Save AI tokens across providers. Single Rust binary that compresses command output + injects caveman-mode prompts.',

  head: [
    ['link', { rel: 'icon', href: '/favicon.svg' }],
  ],

  themeConfig: {
    logo: '/favicon.svg',

    nav: [
      { text: 'Guide', link: '/guide/' },
      { text: 'Commands', link: '/commands/' },
      { text: 'Integrations', link: '/integrations/' },
    ],

    sidebar: {
      '/guide/': [
        { text: 'Guide', items: [
          { text: 'Getting Started', link: '/guide/' },
          { text: 'Installation', link: '/guide/installation' },
          { text: 'Configuration', link: '/guide/configuration' },
          { text: 'How It Works', link: '/guide/how-it-works' },
        ]},
      ],
      '/commands/': [
        { text: 'Commands', items: [
          { text: 'Overview', link: '/commands/' },
          { text: 'Git', link: '/commands/git' },
          { text: 'Rust / Cargo', link: '/commands/cargo' },
          { text: 'Node.js / npm', link: '/commands/npm' },
          { text: 'Testing', link: '/commands/testing' },
          { text: 'Linting', link: '/commands/linting' },
          { text: 'Docker / K8s', link: '/commands/docker' },
          { text: 'Files', link: '/commands/files' },
          { text: 'GitHub CLI', link: '/commands/gh' },
          { text: 'System & Network', link: '/commands/system' },
        ]},
      ],
      '/integrations/': [
        { text: 'Integrations', items: [
          { text: 'Overview', link: '/integrations/' },
          { text: 'Claude Code', link: '/integrations/claude' },
          { text: 'GitHub Copilot', link: '/integrations/copilot' },
          { text: 'Cursor', link: '/integrations/cursor' },
          { text: 'Gemini CLI', link: '/integrations/gemini' },
          { text: 'OpenCode', link: '/integrations/opencode' },
        ]},
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/sonyarianto/miskin' },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2026 Sony AK',
    },

    editLink: {
      pattern: 'https://github.com/sonyarianto/miskin/edit/main/docs/:path',
    },

    search: {
      provider: 'local',
    },
  },
})
