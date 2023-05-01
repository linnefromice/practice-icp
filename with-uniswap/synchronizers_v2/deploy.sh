dfx stop && dfx start --background --clean
dfx deploy reader_v2 \
    --argument '(
        "https://eth-mainnet.alchemyapi.io/v2/JVUDgQSB0r-3HhohPCod6uBy_Zx8WEdy",
        "88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    )'

dfx canister call reader_v2 bulk_save_prices '(
    vec {
        16950602;
        16957716;
        16964816;
        16971894;
        16978915;
        16985956;
        16992952;
        17000010;
        17007072;
        17014117;
        17021181;
        17028241;
        17035258;
        17041766;
        17048771;
        17055791;
        17062833;
        17069898;
        17076967;
        17084047;
        17091074;
        17098172;
        17105309;
        17112440;
        17119571;
        17126694;
        17133820;
        17140929;
        17148051;
        17155163
    },
    null,
    null
)'