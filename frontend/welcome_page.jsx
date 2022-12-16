const header = {
    background: "#33333D",
    height: "263px",
    color: "white",
    fontFamily: 'Inter',
    fontWeight: "400",
    fontSize: "58px",
    textAlign: "center",
    lineHeight: "70px",
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
};


const paragraph = {
    marginLeft: "70px",
    marginRight: "40px",
    marginTop: "47px",
};

const paragraphTitle = {
    fontFamily: 'Inter',
    fontWeight: "600",
    fontSize: "28px",
    lineHeight: "34px",
};

const paragraphContent = {
    marginTop: "26px",
    fontFamily: 'Roboto',
    fontWeight: "400",
    fontSize: "20px",
    lineHeight: "130 %",
    color: "#49454F",
};

const messageBoxes = {
    width: "1400px",
};

const messageBox = {
    display: "inline-block",
    height: "360px",
    width: "350px",
    background: "#F6F6F6",
    borderRadius: "10px",
    marginLeft: "17px",
    marginRight: "17px",
    padding: "35px",
    lineHeight: "32px",
}


return (
    <div>

        <div class="header" style={header}> Ready to get rewarded for achieving your goals?</div>

        <div class="paragraph" style={paragraph}>
            <div style={paragraphTitle}>
                Achieve your goals

            </div>
            <div style={paragraphContent}>
                BetterYou helps you achieve your goals with support from your friends. You commit to a challenge and
                your
                friends will bet NEAR tokens on you. If you win, your friends will get a better version of you and
                reward
                you for it. If you lose, your friends will get a little richer and hold you accountable for your next
                challenge.
            </div>

        </div>
        <div class="paragraph" style={paragraph}>
            <div style={paragraphTitle}>
                How it works
            </div>
            <div style={paragraphContent}>
                <div style={messageBox}>
                    01<br /> <br />
                    <b>Commit to goals</b><br /><br />
                    Browse available challenges and pick the ones you want to commit to. Your friends can then bet NEAR
                    tokens on you
                </div>
                <div style={messageBox}>
                    02<br /><br />
                    <b>Get going</b><br /><br />
                    Check in on a daily basis to track your progress and achieve your goal.
                </div>
                <div style={messageBox}>
                    03<br /><br />
                    <b>Win rewards</b><br /><br />
                    Claim your reward if you achieve your goals.
                </div>
            </div>
        </div>
    </div>
)