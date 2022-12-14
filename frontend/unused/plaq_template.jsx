const challengeState = props.challengeState;



let percentLeft = 100 * parseInt(challengeState["days_left"], 10) / parseInt(challengeState["total_days"], 10);
let daysPercentPassed = 100 - percentLeft;

let skipDaysLeftPercent = 100 * parseInt(challengeState["lives_left"], 10) / parseInt(challengeState["total_lives"], 10);

const challengeDivStyle = {
    width: "300px",
    border: "2px #dee2e6 solid",
    borderRadius: "5px",
};

const internalDivStyle = {
    padding: "20px",
};

const sectionDivStyle = {
    paddingTop: "20px",
    paddingBottom: "20px",
    color: "#68717A",
};

const otherSectionDivStyle = {
    paddingBottom: "20px",
}

const progressBarOutside = {
    backgroundColor: "#e9ecef",
    borderRadius: "5px"
}


const renderProgressBar = (percentFull) => (
    <div style={progressBarOutside}>
        <div style={{ height: 20 + "px", width: percentFull + "%", backgroundColor: "#000000", borderRadius: 5 + "px" }}> </div>
    </div>
);

let reward = challengeState["funding"]["initial_stake"] || 0;
let backers = 0;

if (challengeState["funding"]["backers"]) {
    let array = challengeState["funding"]["backers"];
    for (let index = 0; index < array.length; index++) {
        const element = array[index];
        reward += element["value"];
        backers += 1;
    }
}

let reward_str = Number(reward / 1_000_000_000_000_000_000_000_000).toFixed(1);



let challengeStatus = "";
let currentRewards = "";

if (challengeState["days_left"] == 0) {
    challengeStatus = "success";
    let collectRewardButton = "";
    if (props.accountId == context.accountId) {
        collectRewardButton = (<button>Collect your Reward</button>)
    }
    currentRewards = (
        <div style={otherSectionDivStyle}>
            <h2>Challenge finished succesfully !! </h2>
            <span style={{ fontSize: 26 + "px", fontWeigth: "bold" }}> {reward_str} NEAR </span><br />
            From {backers} people <br />
            {collectRewardButton}

        </div>
    )
} else if (challengeState["lives_left"] == 0) {
    challengeStatus = "failed";
    let resetRewardButton = "";
    if (props.accountId == context.accountId) {
        resetRewardButton = (<button>Reset your challenge - and try again</button>)
    }
    currentRewards = (
        <div style={otherSectionDivStyle}>
            <h2>!! Challenge Failed !!  </h2>
            <span style={{ fontSize: 26 + "px", fontWeigth: "bold" }}> {reward_str} NEAR </span><br />
            From {backers} people <br />

            {resetRewardButton}
        </div>
    )
} else {
    challengeStatus = "inprogress";

    currentRewards = (
        <div style={otherSectionDivStyle}>
            <h2>Current Rewards</h2>
            <span style={{ fontSize: 26 + "px", fontWeigth: "bold" }}> {reward_str} NEAR </span><br />
            From {backers} people <br />
            <button>Send Reward</button>
        </div>
    )
}



return (

    <div>
        <div style={challengeDivStyle}>
            <img src={props.challengeLogo} style={{ width: 100 + "%" }} />
            <div style={internalDivStyle}>
                <div style={sectionDivStyle}>
                    {props.customBox}
                </div>
                <hr />
                <div style={internalDivStyle}>
                    <h2>Progress</h2>
                    Remaining days

                    {renderProgressBar(daysPercentPassed)}
                    <span style={{ float: "right" }}>{challengeState["days_left"]} days</span>
                    <br />

                    Available Skip Days
                    {renderProgressBar(skipDaysLeftPercent)}

                    <span style={{ float: "right" }}>{challengeState["lives_left"]} days</span>

                </div>
                <hr />

                {currentRewards}

                <hr />
                <div style={otherSectionDivStyle}>

                    <h2>Current bets</h2>

                    <div style={{ fontSize: 20 + "px" }}>Success: <b> 200 NEAR </b></div>
                    10 people <br />
                    <button>Bet on success</button>

                    <br /><br />

                    <div style={{ fontSize: 20 + "px" }}>Failure: <b> 40 NEAR </b></div>
                    5 people <br />
                    <button>Bet on failure</button>
                </div>
            </div>
        </div>
    </div>

);