%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: aeneid
Summary: use GitHub as a free, zero-ops Identity Provider
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://nikhiljha.com/projects/

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
%config(noreplace) %{_sysconfdir}/aeneid/config.toml

%post
# create aeneid user
groupadd -r aeneid >/dev/null 2>&1 || :
useradd -M -n -g aeneid -r -d /etc/aeneid -s /bin/bash \
    -c "aeneid user" aeneid >/dev/null 2>&1 || :
# create directories so aeneid can store caches and read config files
install -d -o aeneid -g aeneid /etc/aeneid
chown -R aeneid:aeneid /etc/aeneid
