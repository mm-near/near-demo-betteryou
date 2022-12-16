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



const connectionConfig = {
    networkId: "testnet",
    keyStore: myKeyStore, // first create a key store 
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
};
const nearConnection = await connect(connectionConfig);
const account = await nearConnection.account("language.betteryou.testnet");


const contract = new Contract(
    account, // the account object that is connecting
    "language.betteryou.testnet",
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
        const duo = new Duo(challenge[1].duolingo_username.toString());
        const courses = await duo.getCourses();
        const result = courses.find(({ title }) => title === challenge[1].language);
        if (result) {
            updateArray.push([challenge[0], result.xp]);
        }
    }
    console.log(updateArray);
    try { await contract.admin_update_challenge({ update: updateArray }) } catch (e) {
        console.log(e);
    };
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
    var url = base + encodeURIComponent(`https://www.duolingo.com/2017-06-30/users?username=${self.username}`);
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

//cron job that runs the update on the contract every 10 minutes
cron.schedule('10 * * * *', () => {
    console.log("Running the cron job")
    run();
});
