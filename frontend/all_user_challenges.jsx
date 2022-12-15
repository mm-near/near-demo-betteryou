const accountId = props.accountId ?? context.accountId;


const testnetWidgets = {
    dummyChallenge: "nft12.testnet/widget/dummy_challenge",
    languageChallenge: "nft12.testnet/widget/language_challenge",
};

let logo = (<img src="https://user-images.githubusercontent.com/91919554/207791330-aa5bc143-b51d-49b0-a0be-eae26365f1f2.png" />);

if (props.hideLogo) {
    logo = "";
}

const widgets = testnetWidgets;
return (
    <div>
        {logo}
        <div style={{ display: "table" }}>
            <div style={{ display: "table-row" }}>
                <div style={{ width: 600 + "px", display: "table-cell" }}>
                    <Widget src={widgets["dummyChallenge"]} props={{ accountId }} />
                </div>
                <div style={{ display: "table-cell" }}>
                    <Widget src={widgets["languageChallenge"]} props={{ accountId }} />
                </div>
            </div>
        </div>
    </div>
);