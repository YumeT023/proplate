export const Logo = () => {
  return (
    <h1 className="flex flex-row items-baseline text-2xl font-bold">
      <span className="tracking-tight hover:cursor-pointer light:text-black">
        <span className="text-amber-500">🛠 {`Proplate`}</span>
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
    content: () => null,
  },
  footer: {
    component: () => null,
  },
  titleTemplate: "%s - proplate",
  project: {
    link: "https://github.com/YumeT023/proplate",
  },
  // ... other theme options
};

export default config;
