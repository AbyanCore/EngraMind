reset:
	solana-keygen new --force
	solana airdrop 10
	rm -r target
	anchor keys sync

pbd:
	anchor build
	anchor deploy

pb:
	anchor build

test:
	anchor test --skip-local-validator --skip-build --skip-deploy

test-full:
	anchor test --skip-local-validator

ca:
	solana-keygen new --force --outfile ~/.config/solana/id.json
	solana airdrop 10