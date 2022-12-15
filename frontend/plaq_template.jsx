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

                <div style={otherSectionDivStyle}>
                    <h2>Current Rewards</h2>
                    <span style={{ fontSize: 26 + "px", fontWeigth: "bold" }}> {challengeState["reward"]} NEAR </span><br />
                    From 3 people <br />
                    <button>Send Reward</button>
                </div>

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