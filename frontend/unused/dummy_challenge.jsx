const accountId = props.accountId ?? context.accountId;


const testnetWidgets = {
    challengePlaqTemplate: "nft12.testnet/widget/plaq_template",
};

const widgets = testnetWidgets;


const challengeState = Near.view("dummy.betteryou.testnet", "get_state_for_user", {
    account_id: accountId,
});

if (!challengeState) {
    // TODO: if props.accountId == context.accountId -> then suggest the box to create a new challenge.
    return "No challenges";
}

const customBox = (
    <div>
        <h1 style={{ color: "black" }}> Wake up (mock version) </h1>
        <b>{accountId}</b> is participating in this challenge.
        They have committed to waking up every day for <b>{challengeState["total_days"]} days</b> at < b > 6: 00 AM</b >.
        Do you want to support them ?
    </div>
);

const challengeLogo = "https://user-images.githubusercontent.com/91919554/207791325-65269d93-4ef3-4844-b029-1223099fc816.png";



return (
    <div>
        <Widget src={widgets["challengePlaqTemplate"]} props={{ challengeState, customBox, challengeLogo, accountId }} />
    </div>
);