# -*- mode: ruby -*-
# vi: set ft=ruby :


$install_rust_script = <<SCRIPT
set -e
echo "installing rust"
sudo apt-get install -y curl libssl-dev
curl https://sh.rustup.rs -sSf > ~/rustup.sh
sh ~/rustup.sh -y
echo "rust installed\ninstalling kcov from github"
echo "installing kcov deps"
sudo apt-get -yq --no-install-suggests --no-install-recommends --force-yes install libcurl4-openssl-dev libelf-dev libdw-dev binutils-dev cmake pkgconf build-essential
echo "installing kcov"
mkdir ~/kcov
cd ~/kcov
wget https://github.com/SimonKagstrom/kcov/archive/v32.tar.gz
tar -xf v32.tar.gz --strip 1
mkdir build
cd build
cmake ..
make -j
sudo make install
sudo systemctl reboot
SCRIPT


VAGRANTFILE_API_VERSION = "2"

# vbguest is needed for nfs
unless Vagrant.has_plugin?("vagrant-vbguest")
  raise "See below:\n\n\nNeccessary plugins are not installed! use:\n\n\t`vagrant plugin install vagrant-vbguest`\n\nto install them"
end

Vagrant.configure(VAGRANTFILE_API_VERSION) do |config|

  # disable default shared folder. (default is a RW mount to the host, WTF???)
  config.vm.synced_folder ".", "/vagrant", disabled: true

  config.vm.define "development" do |node|
    node.vm.box = "debian/jessie64"
    node.vbguest.auto_update = true

    node.vm.provider "virtualbox" do |vb|
      vb.memory = 1024
      vb.cpus = 2
    end

    node.vm.network :private_network, ip: "192.166.240.2"
    node.vm.network "forwarded_port", guest: 3000, host: 3000

    node.vm.synced_folder "./", "/home/vagrant/smtp", options: ["rw"], type: "nfs"

    config.vm.provision "shell", inline: $install_rust_script, privileged: false
  end

end
