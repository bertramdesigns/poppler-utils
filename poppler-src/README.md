# Poppler utils build for Rust crate poppler-utils

Version 0.0.1

Because we will not be accessing any of the front-end functionality of Poppler, we can build the library without the Qt5, Qt6, glib, or cpp wrappers.

Download the source code from [poppler.freedesktop.org](https://poppler.freedesktop.org/) and extract to poppler-src.
You will also be able to find poppler-data on the same site. These are the encoding files that enable poppler to correctly render CJK and Cyrillic.

Assumes the following folder structure:

- poppler-utils-rs
  - poppler-src
    - poppler-23.12.0
      - build-win
      - build-test
      - build-mac
      - build-unix
  - poppler-cmake-toolchains
    - mac.cmake
    - unix.cmake
    - win.cmake
    - poppler-data-0.4.12
  - src
    - poppler
      - mac
      - unix
      - win

_**NOTE:** The poppler-test folder is not required for the build, but is useful for testing the library. It can be downloaded from the [Poppler repository](https://gitlab.freedesktop.org/poppler/test). Place the contents in poppler-23.12.0/build-test and run "make test"_

---

The library will need to have executables for all systems it may run on. This should include PE, ELF, and Mach-O. To create these files, Poppler will need to be built with the proper compilers for each.

- [Windows](#build-for-windows-compiled-on-mac-with-mingw-w64)
- [Mac](#build-for-mac)
- [Linux & Unix](#build-for-linux-compiled-with-dev-container)
- Android (TODO)

In general, the build process will look like this:

1. [Build the poppler-data package](#build-the-poppler-data-package)

2. For the target system /build-{target}, cmake pointing to the correct toolchain file

- `-DCMAKE_TOOLCHAIN_FILE=../../poppler-cmake-toolchains/{target}.cmake`

3. `Make`

- optional: `Make test`

4. `Make install`

The report for support should look like this:

```bash
Building Poppler with support for:
  font configuration:   fontconfig
  use boost (Splash):   yes
  cairo output:         yes
  qt5 wrapper:          no
  qt6 wrapper:          no
  glib wrapper:         no
    introspection:      no
    gtk-doc:            no
  cpp wrapper:          no
  use libjpeg:          yes
  use libpng:           yes
  use libtiff:          yes
  use zlib uncompress:  no
  use nss3:             no
  use gpg:              yes
    default signature backend:          NSS
  use curl:             no
  use libopenjpeg2:     yes
  use lcms2:            yes
  command line utils:   yes
  fuzz target:          no
  test data dir:        ../../poppler-test
```

---

## Build the poppler-data package

The encoding files enables poppler to correctly render CJK and Cyrillic properly. You can find the source code download [here](https://poppler.freedesktop.org/).

You will build the data encoding files to your computer. The way CMakeLists.txt is written, it is not easy to override the path and ensure a proper include if it is in the source directory.

```bash
# Navigate to the directory where you've downloaded the Poppler source code
cd /path/to/root/poppler-src/poppler-data-0.4.12

# Clean incase anything is left over from a previous build
make clean

# Install the data files in the source directory top level
make install

```

If you would like to remove these files, go to the install directory (usually /usr/local/share/poppler) and delete the files. The resulting files are not executables.

---

## Build for Windows (compiled on Mac with mingw-w64)

Assuming the projects is being compiled on a Mac using mingw-w64 installed via homebrew, use the following commands to configure the build:

### 1. Install mingw-w64 (and clang-format)

Note the version number for CMAKE_FIND_ROOT_PATH below. I am currently using 11.0.1

If you already have mingw-w64 installed, consider reinstalling and starting fresh to avoid outdated libraries.

```bash
# Add mingw-w64 with homebrew
brew install mingw-w64
brew install clang-format
```

**NOTE:**
When running, it is likely that you will get an error about a missing libraries. Download them from [MSYS2](https://packages.msys2.org/search?q=mingw-w64) and merge into the corresponding folders in `/usr/local/Cellar/mingw-w64/11.0.1/toolchain-x86_64/x86_64-w64-mingw32`. Included in this repo is the toolchain-x86_64 folder with all the needed libraries that can be MERGED (not replaced), but it is recommended to reinstall updated versions when recompiling future versions of Poppler.

_<sup>\*\*</sup> In this build, CURL and NSS are being disabled. Mingw-w64 does not have sys/socket. If the functionalities are needed compile natively on Windows VM or modify the source to work with a Mingw-w64 compatible socket library. Make sure to enable the feature on all builds to make it uniform._

### 2. Navigate and build the directories

```bash
# Close and reopen your terminal to make sure the new environment variables are loaded

# Navigate to the directory where you've downloaded the Poppler source code
cd /path/to/root/poppler-src/poppler-23.12.0

# Create a new directory for the build
mkdir build-win
cd build-win
```

### 4. Configure the build

```bash
# Clean incase anything is left over from a previous build
make clean

# Configure the build
cmake .. -DCMAKE_TOOLCHAIN_FILE=../../poppler-cmake-toolchains/win.cmake
```

### 5. Build and install the library:

```bash

# clean the existing install directory
make clean
rm -rf ../../../src/poppler/win

# Build the project
make

# Install the library to a specified location
# Prefix is defined in DCMAKE_INSTALL_PREFIX
make install DESTDIR=../../../

```

Check the build directory to make sure all the files are .dll or .exe files. If they are Mach-O or ELF, the build was not configured correctly (probably from make clean not resetting the build directory).

---

## Build for Mac

### 1. Navigate and build the directories

```bash
# Close and reopen your terminal to make sure the new environment variables are loaded

# Navigate to the directory where you've downloaded the Poppler source code
cd /path/to/root/poppler-src/poppler-23.12.0

# Create a new directory for the build
mkdir build-mac
cd build-mac
```

### 2. Configure the build

```bash
# Clean incase anything is left over from a previous build
make clean

# Configure the build
cmake .. -DCMAKE_TOOLCHAIN_FILE=../../poppler-cmake-toolchains/mac.cmake

```

### 3. Build and install the library:

```bash

# clean the existing install directory
rm -rf ../../../src/poppler/mac

# Build the project
make

# Optional: Run the tests
make test

# Install the library to a specified location
# Prefix is defined in DCMAKE_INSTALL_PREFIX
make install DESTDIR=../../../

```

Make sure all the files are Mach-O files.

---

## Build for Linux (compiled with dev container)

It is easiest to build the library using the vscode dev containers with a linux image.

### 1. Setup dev container

The following should be installed:

- Docker Desktop
- [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension for vscode

Startup the dev container:

1. Open the root folder in vscode
2. Click the blue button in the bottom left corner of vscode

   - If you haven't built the container yet, select "New Dev Container"
     - Choose C++ and Debian without additional features.
   - If you have built the container, select "Reopen in Container"

3. Make a place to do all the building

```bash
# Navigate to the directory where you've downloaded the Poppler source code
cd /path/to/root/poppler-src/poppler-23.12.0

# Create a new directory for the build
mkdir build-unix
cd build-unix
```

### 2. Install the required packages

The C++ version of the container has quite a few of the required packages already installed, but a few more are needed.

```bash
# If you have a fresh container make sure to update apt
sudo apt-get update
```

Annoyingly, GPGME++ is quite outdated in the apt repository so it must be built from source.

```bash
# Download the source
sudo git clone https://github.com/gpg/gpgme.git

# Navigate to the cloned directory
cd gpgme

# Install the build dependencies
sudo apt-get install -y automake libtool pkg-config libglib2.0-dev python-dev libgpg-error-dev libassuan-dev zlib1g-dev libbz2-dev texinfo autoconf

# Generate the config file
sudo sh autogen.sh

# Configure the build, making sure to only build for cpp
sudo ./configure --enable-languages=cpp

# make, install
sudo make
sudo make install

```

Now for the other packages needed for poppler:

```bash
sudo apt-get install -y libfreetype6-dev libfontconfig1-dev libopenjp2-7-dev libjpeg-dev libpng-dev libtiff-dev libcairo2-dev libboost-dev liblcms2-dev
```

### 2. Configure the build

Make sure to move back up to the build directory before running cmake. It is recommended at this point to restart the terminal.

```bash
# back to the build directory
cd /path/to/root/poppler-src/poppler-23.12.0/build-unix

# Clean incase anything is left over from a previous build
make clean

# Configure the build
cmake .. -DCMAKE_TOOLCHAIN_FILE=../../poppler-cmake-toolchains/unix.cmake

```

### 3. Build and install the library:

```bash

# clean the existing install directory
make clean
rm -rf ../../../src/poppler/unix

# Build the project
make

# Install the library to a specified location
# Prefix is defined in DCMAKE_INSTALL_PREFIX
make install DESTDIR=../../../

```

Check the build directory to make sure all the files are ELF files.
