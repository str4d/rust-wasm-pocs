import { Client } from "zcash-client-sdk-js";

const client = Client.new();

const address = document.getElementById("zcash-client-address");
address.textContent = client.address();

const balance = document.getElementById("zcash-client-balance");
const noBalance = document.getElementById("zcash-client-no-balance");
const updateBalance = () => {
    var newBalance = client.balance();
    balance.textContent = `Balance: ${newBalance} TAZ`;
    if (newBalance > 0) {
        noBalance.style.display = "none";
    } else {
        noBalance.style.display = "";
    }
};
updateBalance();

// Loading complete, show the wallet
document.getElementById("zcash-client-loading").remove();
document.getElementById("zcash-client-content").style.display = "";
