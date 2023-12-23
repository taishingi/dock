FROM archlinux/archlinux
RUN pacman -Syyu --noconfirm wget curl base-devel git go fd go-tools revive staticcheck  goreleaser yaegi gopls delve gox eza
RUN export PATH="$PATH:$(go env GOBIN):$(go env GOPATH)/bin"