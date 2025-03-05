class Makedir < Formula
  desc "Better mkdir: Create directories with predefined setups"
  homepage "https://github.com/michaelsousajr/makedir"
  license "MIT"

  on_macos do
    url "https://github.com/michaelsousajr/makedir/releases/download/v0.1.0/makedir-mac.tar.gz"
    sha256 "a1ac4847e5af4cd4d68f9db4dcfb475bb75f6821abdb1b80adf73c15b50209a6"
  end

  def install
    bin.install "makedir"
  end

  test do
    system "#{bin}/makedir", "--help"
  end
end
