import "./globals.css";

export const metadata = {
  title: "ClashAI",
  description: "Legal AI companion for Clash of Clans players and clans.",
  icons: { icon: "/favicon.svg", shortcut: "/favicon.svg" },
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="en"><body>{children}</body></html>;
}
