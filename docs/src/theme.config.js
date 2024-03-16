import { useRouter } from "next/router";
import { useConfig } from "nextra-theme-docs";

const Logo = () => {
  return (
    <h1 className="flex flex-row items-baseline text-2xl font-bold">
      <span className="tracking-tight hover:cursor-pointer light:text-black">
        <span className="font-bold text-amber-600">Proplate</span>
        <span className="font-light"> Lazy gen devtools</span>
      </span>
    </h1>
  );
};

const Head = () => {
  const { asPath, defaultLocale, locale } = useRouter();
  const { frontMatter } = useConfig();
  const url =
    "https://proplate.vercel.app" +
    (defaultLocale === locale ? asPath : `/${locale}${asPath}`);

  return (
    <>
      <meta property="og:url" content={url} />
      <meta
        property="og:title"
        content={frontMatter.title || "Proplate - Lazy gen devtools"}
      />
      <meta
        property="og:description"
        content={
          frontMatter.description ||
          "Docs for the best tools to begin your project with"
        }
      />
    </>
  );
};

/**
 * @type {import('nextra-theme-docs').DocsThemeConfig}
 */
const config = {
  logo: Logo,
  head: Head,
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
