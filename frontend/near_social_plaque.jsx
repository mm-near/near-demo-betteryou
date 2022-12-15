const accountId = props.accountId ?? context.accountId;

const score = Near.view("dummy.betteryou.testnet", "get_state_for_user", {
  account_id: accountId,
});

if (!score) {
  return "No challenges";
}


let percentLeft = 100 * parseInt(score["days_left"], 10) / parseInt(score["total_days"], 10);
let daysPercentPassed = 100 - percentLeft;

let skipDaysLeftPercent = 100 * parseInt(score["lives_left"], 10) / parseInt(score["total_lives"], 10);

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

let logo = (<img src="https://user-images.githubusercontent.com/91919554/207791330-aa5bc143-b51d-49b0-a0be-eae26365f1f2.png" />
);

if (props.hideLogo) {
  logo = "";
}

const renderProgressBar = (percentFull) => (
  <div style={progressBarOutside}>
    <div style={{ height: 20 + "px", width: percentFull + "%", backgroundColor: "#000000", borderRadius: 5 + "px" }}> </div>
  </div>
);


return (

  <div>
    {logo}

    <div style={challengeDivStyle}>
      <img src="https://user-images.githubusercontent.com/91919554/207791325-65269d93-4ef3-4844-b029-1223099fc816.png" style={{ width: 100 + "%" }} />
      <div style={internalDivStyle}>
        <div style={sectionDivStyle}>
          <h1 style={{ color: "black" }}> Wake up </h1>
          <b>{accountId}</b> is participating in this challenge.
          They have committed to waking up every day for <b>{score["total_days"]} days</b> at <b>6:00 AM</b>.
          Do you want to support them?
        </div>
        <hr />
        <div style={internalDivStyle}>
          <h2>Progress</h2>
          Remaining days

          {renderProgressBar(daysPercentPassed)}
          <span style={{ float: "right" }}>{score["days_left"]} days</span>
          <br />

          Available Skip Days
          {renderProgressBar(skipDaysLeftPercent)}

          <span style={{ float: "right" }}>{score["lives_left"]} days</span>

        </div>
        <hr />

        <div style={otherSectionDivStyle}>
          <h2>Current Rewards</h2>
          <span style={{ fontSize: 26 + "px", fontWeigth: "bold" }}> {score["reward"]} NEAR </span><br />
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