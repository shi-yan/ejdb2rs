docker run -it --rm \
    --workdir="/home/$USER" \
    -v="/etc/group:/etc/group:ro" \
    -v="/etc/passwd:/etc/passwd:ro" \
    -v="/etc/shadow:/etc/shadow:ro" \
    --network host \
    -e="DISPLAY=$DISPLAY" \
    -e "TERM=xterm-256color" \
    -v="/tmp/.X11-unix:/tmp/.X11-unix" \
    -v="/tmp/host_launcher_fifo:/tmp/host_launcher_fifo" \
    -v /home/shiy/ejdb_bindings:/home/shiy/ejdb_bindings \
    -v /home/shiy/dummy_local:/home/shiy/.local \
    -v /home/shiy/.cargo:/home/shiy/.cargo \
    -v /home/shiy/.rustup:/home/shiy/.rustup \
 -u $(id -u ${USER}):$(id -g ${USER}) \
 quay.io/pypa/manylinux2014_x86_64