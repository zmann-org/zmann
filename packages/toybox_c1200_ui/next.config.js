/** @type {import('next').NextConfig} */
const nextConfig = {
  eslint: {
    ignoreDuringBuilds: true,
  },
  typescript: {
    ignoreBuildErrors: true,
  },
  compress: true,
  output: "export",
  distDir: "dist",
  transpilePackages: [],
};

module.exports = nextConfig;
