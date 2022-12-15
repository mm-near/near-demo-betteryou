const accountId = props.accountId ?? context.accountId;


const testnetWidgets = {
    challengePlaqTemplate: "nft12.testnet/widget/plaq_template",
};

const widgets = testnetWidgets;

// TODO: change to a different contract
const challengeState = Near.view("dummy.betteryou.testnet", "get_state_for_user", {
    account_id: accountId,
});

if (!challengeState) {
    // TODO: if props.accountId == context.accountId -> then suggest the box to create a new challenge.
    return "No challenges";
}

const customBox = (
    <div>
        <h1 style={{ color: "black" }}> Language (mock version) </h1>
        <b>{accountId}</b> is participating in this challenge.
        They have committed to learn <b>German</b> every day for <b>{challengeState["total_days"]} days</b>
        Do you want to support them ?
    </div>
);
const challengeLogo = "https://user-images.githubusercontent.com/91919554/207844104-38c943cf-4790-4e71-b6bc-09a09a61cf6e.png";


return (
    <div>
        <Widget src={widgets["challengePlaqTemplate"]} props={{ challengeState, customBox, challengeLogo, accountId }} />
    </div>
);