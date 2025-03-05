class Makedir < Formula
  desc "Better mkdir: Create directories with predefined setups"
  homepage "https://github.com/michaelsousajr/makedir"
  url "https://github.com/yourusername/makedir/releases/download/v0.1.0/makedir-mac.tar.gz"
  sha256 "15d9a1faaf993a3c25189b3060dd7d9d7dccd928e74b38344b6f4a7e87e1086d"
  license "MIT"

  def install
    bin.install "makedir"
  end

  test do
    system "#{bin}/makedir", "--help"
  end
end
