const challengeLogo =
    "https://user-images.githubusercontent.com/91919554/207844104-38c943cf-4790-4e71-b6bc-09a09a61cf6e.png";
const plaqTemplateWidget = "nft12.testnet/widget/plaq_template";
const contractId = "wakeup.betteryou.testnet";
const accountId = props.accountId ?? context.accountId;

let contractChallengeState = None;

if (!accountId) {
    return "Please log in to view your own challenge or specify account id to view in props.";
}

function isOwnerLoggedIn() {
    return accountId == context.accountId;
}

function get_first_day() {
    const now = new Date();
    const tomorrow_six_am = new Date(
        now.getFullYear(),
        now.getMonth(),
        now.getDate() + 1,
        06,
        00,
        00
    );
    console.log("tomorrow", tomorrow_six_am);
    return Math.floor(tomorrow_six_am.getTime() / 1000);
}

function create_challenge() {
    console.log("create challenge");

    Near.call(contractId, "create_challenge", {
        config: {
            lives: 2,
            days: 7,
            first_day: get_first_day(),
            timeout: 4 * 60 * 60,
        },
    });
}

function updateContractChallengeState() {
    contractChallengeState = Near.view(contractId, "get_challenge", {
        account_id: accountId,
    });
}

function create_test_challenge() {
    const first_day = Math.floor(Date.now() / 1000) + 10;
    console.log("create test challenge", Date(first_day));

    Near.call(contractId, "create_challenge", {
        config: {
            lives: 3,
            days: 5,
            first_day: first_day,
            timeout: 30,
            day_length: 60,
        },
    });
}

// Update challenge should be called when user wants to register a wakeup.
function updateChallenge() {
    console.log("update challenge");
    const result = Near.call(contractId, "update_challenge", {});
    console.log("update challenge result", result);
    updateContractChallengeState();
    // TOOD reload the page ideally
}

function finish_challenge() {
    console.log("finish challenge");
    const result = Near.call(contractId, "finish_challenge", {});
    console.log("finish result", result);
}

function abandon_challenge() {
    console.log("abandon challenge");
    Near.call(contractId, "abandon_challenge", {});
}

// Conversion function from the contract state format to the widget state format.
function toWidgetChallengeState(contractChallengeState) {
    let result = {};
    result.total_days = contractChallengeState.config.days;
    result.total_lives = contractChallengeState.config.lives;

    result.days_left = result.total_days - contractChallengeState.days_passed;
    result.lives_left = result.total_lives - contractChallengeState.lives_used;

    return result;
}

function getPublicNoChallenge() {
    return "No challenge found.";
}

function getOwnerNoChallenge() {
    return (
        <div>
            <h3>No challenge found, would you like to create a new challenge?</h3>
            <br />
            <button onClick={create_challenge}>Create challenge</button> <br />
            <br />
            <button onClick={create_test_challenge}>Create test challenge</button>
        </div>
    );
}

function getNoChallenge() {
    if (isOwnerLoggedIn()) {
        return getOwnerNoChallenge();
    } else {
        return getPublicNoChallenge();
    }
}

function getPublicCustomBox() {
    return (
        <div>
            <h1 style={{ color: "black" }}> Wake up challenge! </h1>
            <b>{accountId}</b> is participating in this challenge. They have committed
            to wake up early for <b>{contractChallengeState.config.days} days.</b>
            Do you want to support them?
        </div>
    );
}

function time_diff_to_string(time_diff) {
    const hours = Math.floor(time_diff / 60 / 60);
    const minutes = Math.floor((time_diff / 60) % 60);
    const seconds = Math.floor(time_diff % (60 * 60));
    return `${hours}h ${minutes}m ${seconds}s`;
}

function getWakeUpButton() {
    const config = contractChallengeState.config;
    const button_msg = "I'm awake!";
    const day_length = config.day_length;
    const timeout = config.timeout;
    const start = config.first_day;

    const now = Math.floor(Date.now() / 1000);
    if (now < start) {
        const start_date = new Date(start);
        const start_string = start_date.toString();
        return <div> The challenge will begin on {start_string} </div>;
    }

    // Recalculate days passed in case the user missed out some days.
    const time_since_start = now - start;
    const days_passed = Math.floor(time_since_start / day_length);

    console.log("days passed", days_passed);
    console.log("total days", config.days);

    // The challenge is over.
    // - the actual days_passed is larger than the total number of days - can't do equal because user needs to register wake up on the last day.
    // - the contract state days passed is equal to total number of days - we're finished and contract knows about it
    // - the user is out of lives
    if (
        config.days < days_passed ||
        config.days == contractChallengeState.days_passed ||
        config.lives <= contractChallengeState.lives_used
    ) {
        return (
            <div>
                <div>
                    The challenge has now finished. Please press the finish button to wrap
                    it up.
                </div>
                <br />
                <button onClick={finish_challenge}> Finish </button>
            </div>
        );
    }

    // The user has already registered wake up today.
    if (days_passed < contractChallengeState.days_passed) {
        const time_left = start + (days_passed + 1) * day_length - now;
        return (
            <div>
                <div>
                    Well done, you have registered your wake up today. Come back in{" "}
                    {time_diff_to_string(time_left)}.
                </div>
                <br />
                <button disabled={true} onClick={updateChallenge}>
                    {button_msg}
                </button>
            </div>
        );
    }

    // We're counting time since the start time.
    const time_of_day = now - start - days_passed * day_length;
    if (time_of_day < timeout) {
        const time_left = timeout - time_of_day;
        return (
            <div>
                <div>
                    You can register your wake up now. You have
                    {time_diff_to_string(time_left)} to register the wakeup today.
                </div>{" "}
                <br />
                <button onClick={updateChallenge}> {button_msg} </button>
            </div>
        );
    } else {
        const time_left = start + (days_passed + 1) * day_length - now;
        return (
            <div>
                <div>
                    It's too late to register the wake up today. Come back in
                    {time_diff_to_string(time_left)}.
                </div>
                <br />
                <button disabled={true} onClick={updateChallenge}>
                    {button_msg}
                </button>
            </div>
        );
    }
}

function getAbandonButton() {
    return <button onClick={abandon_challenge}> Abandon challenge </button>;
}

function getOwnerCustomBox() {
    return (
        <div>
            <h1 style={{ color: "black" }}> Wake up challenge! </h1>
            <b>You</b> are participating in this challenge. You have committed to wake
            up early for <b>{contractChallengeState.config.days} days.</b> <br />
            <br />
            {getWakeUpButton()} <br />
            {getAbandonButton()} <br />
        </div>
    );
}

function getCustomBox() {
    if (isOwnerLoggedIn()) {
        return getOwnerCustomBox();
    } else {
        return getPublicCustomBox();
    }
}

updateContractChallengeState();

if (!contractChallengeState) {
    return getNoChallenge();
}

console.log("challenge state ", contractChallengeState);

const widgetChallengeState = toWidgetChallengeState(contractChallengeState);

return (
    <div>
        <Widget
            src={plaqTemplateWidget}
            props={{
                challengeState: widgetChallengeState,
                customBox: getCustomBox(),
                challengeLogo: challengeLogo,
            }}
        />
    </div>
);
