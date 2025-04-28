

ferris-backend:
	cargo build --release --bin ferris-backend

ferris-frontend:
	cd ferris-fronend; trunk --config ferris-frontend/trunk.toml build

.PHONY : all
all: ferris-frontend, ferris-backend

systemd:
	echo "[Unit]\nDescription=Ferris-chan server service\nAfter=network.target\nStartLimitIntervalSec=0\n\n[Service]\nType=simple\nRestart=always\nRestartSec=1\nUser=www-data\nExecStart=/usr/bin/ferris-backend\n\n[Install]WantedBy=multi-user.target" > out/ferrischan.service


install-shared:
	sudo mkdir /var/ferris-chan


install-systemd: systemd, all, frontend-destination.txt, install-shared
	sudo cp out/ferrischan.service /etc/systemd/system/
	sudo cp target/release/ferris-backend /usr/bin/
	sudo mkdir "/var/www/$(cat frontend-destination.txt)"
	sudo cp ferris-frontend/dist/. "/var/www/$(cat frontend-destination.txt)"
