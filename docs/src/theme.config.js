import { useRouter } from "next/navigation";

export const Logo = () => {
  return (
    <h1 className="flex flex-row items-baseline text-2xl font-bold">
      <span className="tracking-tight hover:cursor-pointer light:text-black">
        <span className="font-bold text-amber-600">Proplate</span>
        <span className="font-light"> Lazy gen devtools</span>
      </span>
    </h1>
  );
};

/**
 * @type {import('nextra-theme-docs').DocsThemeConfig}
 */
const config = {
  logo: Logo,
  darkMode: true,
  primaryHue: 44,
  nextThemes: {
    defaultTheme: "light",
  },
  feedback: {
    content: () => null,
  },
  editLink: {
    component: () => null,
  },
  footer: {
    component: () => null,
  },
  project: {
    link: "https://github.com/YumeT023/proplate",
  },
  useNextSeoProps: () => {
    const { asPath } = useRouter();
    if (asPath !== "/") {
      return {
        titleTemplate: "%s - Proplate",
      };
    }
  },
  // ... other theme options
};

export default config;
