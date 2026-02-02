import type { Config } from 'tailwindcss';
import typography from '@tailwindcss/typography';

const linkColor = "blue";
const accent = "fg2";

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {
      typography: (theme) => ({
        DEFAULT: {
          css: {
            "--tw-prose-body": theme("colors.fg0"),
            "--tw-prose-headings": theme(`colors.fg0`),
            "--tw-prose-lead": theme("colors.fg0"),
            "--tw-prose-links": theme(`colors.${linkColor}`),
            "--tw-prose-bold": theme(`colors.${accent}`),
            "--tw-prose-counters": theme(`colors.${accent}`),
            "--tw-prose-bullets": theme(`colors.${accent}`),
            "--tw-prose-hr": theme(`colors.${accent}`),
            "--tw-prose-quotes": theme(`colors.${accent}`),
            "--tw-prose-quote-borders": theme(`colors.${accent}`),
            "--tw-prose-captions": theme(`colors.${accent}`),
            "--tw-prose-code": theme(`colors.${accent}`),
            "--tw-prose-pre-code": theme(`colors.${accent}`),
            "--tw-prose-pre-bg": theme(`colors.bg`),
            "--tw-prose-th-borders": theme(`colors.${accent}`),
            "--tw-prose-td-borders": theme(`colors.${accent}`),
            "--tw-prose-invert-body": theme(`colors.${accent}`),
            "--tw-prose-invert-headings": theme("colors.white"),
            "--tw-prose-invert-lead": theme(`colors.${accent}`),
            "--tw-prose-invert-links": theme("colors.white"),
            "--tw-prose-invert-bold": theme("colors.white"),
            "--tw-prose-invert-counters": theme(`colors.${accent}`),
            "--tw-prose-invert-bullets": theme(`colors.${accent}`),
            "--tw-prose-invert-hr": theme(`colors.${accent}`),
            "--tw-prose-invert-quotes": theme(`colors.${accent}`),
            "--tw-prose-invert-quote-borders": theme(
              `colors.${accent}`,
            ),
            "--tw-prose-invert-captions": theme(`colors.${accent}`),
            "--tw-prose-invert-code": theme("colors.white"),
            "--tw-prose-invert-pre-code": theme(`colors.${accent}`),
            "--tw-prose-invert-pre-bg": "rgb(0 0 0 / 50%)",
            "--tw-prose-invert-th-borders": theme(
              `colors.${accent}`,
            ),
            "--tw-prose-invert-td-borders": theme(
              `colors.${accent}`,
            ),
          },
        },
      }),
    },

    colors: {
      "bg": "#282828",
      "bg0_h": "#1d2021",
      "bg1": "#3c3836",
      "bg2": "#504945",
      "fg0": "#fbf1c7",
      "fg1": "#ebdbb2",
      "fg2": "#d5c4a1",
      "red": "#fb4934",
      "yellow": "#fabd2f",
      "blue": "#83a598",
    }
  },

  plugins: [
    typography()
  ],
} satisfies Config;
