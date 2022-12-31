//import Duo from "duo-wrapper";
import fetch from "node-fetch";
var base = "https://api.allorigins.win/get?url=";
import cron from 'node-cron';
import * as nearAPI from "near-api-js";
import * as os from "os";
import * as path from "path";
const { connect } = nearAPI;
const { Contract } = nearAPI;

const { keyStores } = nearAPI;
const homedir = os.homedir();
const CREDENTIALS_DIR = ".near-credentials";
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const myKeyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);



const testnetConnectionConfig = {
    networkId: "testnet",
    keyStore: myKeyStore, // first create a key store 
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
};

const testnetContractAccount = "dev-1671218049971-36407954871979"


const mainnetConnectionConfig = {
    networkId: "mainnet",
    keyStore: myKeyStore, // first create a key store 
    nodeUrl: "https://rpc.near.org",
    walletUrl: "https://wallet.near.org",
    helperUrl: "https://helper.near.org",
    explorerUrl: "https://explorer.near.org",
};

const mainnetContractAccount = "language.betteryou.near"



const connectionConfig = testnetConnectionConfig;
const contractAccount = testnetContractAccount;


const nearConnection = await connect(connectionConfig);
const account = await nearConnection.account(contractAccount);


const contract = new Contract(
    account, // the account object that is connecting
    contractAccount,
    {
        // name of contract you're connecting to
        viewMethods: ["get_all_state"], // view methods do not change state but usually return a value
        changeMethods: ["admin_update_challenge"], // change methods modify state
    }
);

async function run() {

    let updateArray = [];
    let state = await contract.get_all_state();
    for (const challenge of state) {
        try {
            const duo = new Duo(challenge[1].duolingo_username.toString());

            const courses = await duo.getCourses();
            const result = courses.find(({ title }) => title === challenge[1].language);
            if (result) {
                if (result.xp == challenge[1].total_xp) {
                    console.log("No update for " + challenge[0] + " still at " + result.xp);
                } else {
                    updateArray.push([challenge[0], result.xp]);
                }
            } else {
                console.error(`Cannot find language ${challenge[1].language} for user ${challenge[1].duolingo_username} (${challenge[0]})`)
            }
        } catch (error) {
            console.error(`Failed to fetch Duolingo for ${challenge[1].language} for user ${challenge[1].duolingo_username} (${challenge[0]})`)
        }

    }
    console.log(updateArray);
    // Call the contract only if there are some updates.
    if (updateArray.length > 0) {
        try { await contract.admin_update_challenge({ update: updateArray }) } catch (e) {
            console.log(e);
        };
    } else {
        console.log("Skipping");
    }
}


// DUO
function Duo(username) {
    if (!username) {
        throw new Error("Username is required");
    }

    var self = this;
    self.username = username;
    self.params = {
        referrerPolicy: "strict-origin-when-cross-origin",
        body: null,
        method: "GET",
        mode: "cors",
        credentials: "omit",
    };
}
/**
 * Send a request to the Duolingo API
 * @param {Duo} self
 * @param {string} url
 * @returns {Promise}
*/
function request(self, url) {
    return fetch(url, self.params).then(res => {
        return res.json();
    }).catch(err => {
        return Promise.reject(err);
    });
}

/**
 * Parses data from Duolingo API
 * @param {Duo} self
 * @returns {Promise}
*/
function getData(self) {
    var url = base + encodeURIComponent(`https://www.duolingo.com/2017-06-30/users?username=${self.username}&ts=${new Date().getTime()}`);
    return request(self, url).then(res => {
        var data = JSON.parse(res.contents).users[0];
        if (!data) throw new Error("Invalid username");
        return data;
    });
}

/**
 * Gets the user's courses
 * @returns {array}
*/
Duo.prototype.getCourses = function () {
    return getData(this).then(res => {
        return res.courses;
    });
}



console.log("starting first run.");
run();

let date_ob = new Date();
console.log(`Setting up cron ${date_ob.toUTCString()}`);

//cron job that runs the update on the contract every 10 minutes
cron.schedule('*/10 * * * *', () => {
    let date_ob = new Date();

    console.log(`Running the cron job ${date_ob.toUTCString()}`)
    run();
});
