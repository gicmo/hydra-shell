#!/bin/bash

SSHDCFG="$PWD/sshd.cfg"
BINDIR="$1"
BINARY=`realpath $BINDIR/hydra`

if [ ! -x "$BINARY" ]; then
   echo "$BINARY does not exist (or is not executable)"
   exit -1
fi

HOSTKEY="$PWD/ssh_host_rsa_key"

if [ ! -e "$HOSTKEY" ]; then
    ssh-keygen -t rsa -f "$HOSTKEY" -P ""
fi

SSHD=`which sshd`
AUTHKEYS="$PWD/ssh_authorized_keys"
echo "hydra @ $BINARY"
echo "sshd @ $SSHD"
echo "pwd @ $PWD"
echo "cfg @ $SSHDCFG"
echo "host key @ $HOSTKEY"
echo "auth @ $AUTHKEYS"

> "$AUTHKEYS"

for i in client_*.pub; do
    sshkey=$(<"$i")
    clientid="${i%.*}"
    echo "adding $clientid"
    echo "command=\"$BINARY $clientid\" $sshkey" >> "$AUTHKEYS"
done

cat << EOF > "$SSHDCFG"
Port 22222
AddressFamily inet
HostKey $HOSTKEY
UsePrivilegeSeparation no
AuthorizedKeysFile $AUTHKEYS
UsePam no
PidFile $PWD/sshd.pid
EOF

"$SSHD" -De -f "$SSHDCFG"
