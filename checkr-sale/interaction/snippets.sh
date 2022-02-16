PEM_FILE="../../../wallet/wallet-owner.pem"
CHECKR_SALE_CONTRACT="../output/checkr-sale.wasm"

PROXY_ARGUMENT="--proxy=https://gateway.elrond.com"
CHAIN_ARGUMENT="--chain=1"

build_checkr_sale() {
    (set -x; erdpy --verbose contract build "$CHECKR_SALE_CONTRACT")
}

deploy_checkr_sale() {
    local MAX_AMOUNT=1075268817000000000
    local MIN_AMOUNT=1
    local INITIAL_PRICE=4650000000
    local TOKEN_ID=0x434845434b522d363031303862
    
    local OUTFILE="out.json"
    (set -x; erdpy contract deploy --metadata-payable --bytecode="$CHECKR_SALE_CONTRACT" \
        --pem="$PEM_FILE" \
        $PROXY_ARGUMENT $CHAIN_ARGUMENT \
        --outfile="$OUTFILE" --recall-nonce --gas-limit=60000000 \
        --arguments ${MAX_AMOUNT} ${MIN_AMOUNT} ${INITIAL_PRICE} ${TOKEN_ID} --send \
        || return)

    local RESULT_ADDRESS=$(erdpy data parse --file="$OUTFILE" --expression="data['emitted_tx']['address']")
    local RESULT_TRANSACTION=$(erdpy data parse --file="$OUTFILE" --expression="data['emitted_tx']['hash']")

    echo ""
    echo "Deployed contract with:"
    echo "  \$RESULT_ADDRESS == ${RESULT_ADDRESS}"
    echo "  \$RESULT_TRANSACTION == ${RESULT_TRANSACTION}"
    echo ""
}

number_to_u64() {
    local NUMBER=$1
    printf "%016x" $NUMBER
}


deploy_checkr_sale