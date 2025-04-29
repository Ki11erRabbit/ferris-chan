
target/release/ferris-backend:
	cargo build --release --bin ferris-backend

ferris-backend: target/release/ferris-backend

ferris-frontend/dist:
	cd ferris-fronend; trunk --config ferris-frontend/trunk.toml build

ferris-frontend: ferris-frontend/dist

.PHONY : all
all: ferris-frontend ferris-backend

ferrischan.service:
	echo "[Unit]\nDescription=Ferris-chan server service\nAfter=network.target\nStartLimitIntervalSec=0\n\n[Service]\nType=simple\nRestart=always\nRestartSec=1\nUser=www-data\nExecStart=/usr/bin/ferris-backend\n\n[Install]\nWantedBy=multi-user.target" > ferrischan.service
	chmod 0644 ferrischan.service
	chown root:root ferrischan.service


systemd: ferrischan.service ;

install-shared:
	sudo mkdir /var/ferris-chan


install-systemd: systemd all frontend-destination.txt install-shared
	sudo cp ferrischan.service /etc/systemd/system/
	sudo cp target/release/ferris-backend /usr/bin/
	sudo mkdir "/var/www/$(cat frontend-destination.txt)"
	sudo cp ferris-frontend/dist/. "/var/www/$(cat frontend-destination.txt)"

