const accountId = props.accountId ?? context.accountId;


const testnetWidgets = {
    challengePlaqTemplate: "nft12.testnet/widget/plaq_template",
};

const widgets = testnetWidgets;

let challengeState = Near.view("wakeup.betteryou.testnet", "get_challenge", {
    account_id: accountId,
});

if (!challengeState) {
    // TODO: if props.accountId == context.accountId -> then suggest the box to create a new challenge.
    return "No challenges";
}

// Adapt the variable names
challengeState["total_days"] = challengeState["config"]["days"];
challengeState["total_lives"] = challengeState["config"]["lives"];
challengeState["days_left"] = challengeState["config"]["days"] - challengeState["days_passed"];
challengeState["lives_left"] = challengeState["config"]["lives"] - challengeState["lives_used"];


let d = new Date(challengeState["config"]["first_day"] * 1000);
let startTime =
    d.getHours().toString() + ":" + d.getMinutes().toString().padStart(2, "0");

let d1 = new Date(
    (challengeState["config"]["first_day"] +
        challengeState["config"]["timeout"]) *
    1000
);
let endTime =
    d1.getHours().toString() + ":" + d1.getMinutes().toString().padStart(2, "0");



const customBox = (
    <div>
        <h1 style={{ color: "black" }}> Wake up</h1>
        <b>{accountId}</b> is participating in this challenge.
        They have committed to waking up every day for <b>{challengeState["total_days"]} days</b> between <b>{startTime} - {endTime}</b>.
        Do you want to support them ?
    </div>
);
const challengeLogo = "https://user-images.githubusercontent.com/91919554/207791325-65269d93-4ef3-4844-b029-1223099fc816.png";


return (
    <div>
        <Widget src={widgets["challengePlaqTemplate"]} props={{ challengeState, customBox, challengeLogo, accountId }} />
    </div>
);