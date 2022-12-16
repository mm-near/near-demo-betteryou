const signUp = {
    fontFamily: 'Inter',
    background: "#F7F7F7",
    width: "800px",
    borderRadius: "10px",
    textAlign: "center",
    padding: "20px",
    alignItems: "center",
}

const challengeTitle = {
    padding: "20px",
    fontFamily: 'Roboto',
    fontStyle: "normal",
    fontWeight: "400",
    fontSize: "44px",
    lineHeight: "32px",
}

const challengeSubTitle = {
    fontFamily: 'Roboto',
    fontStyle: "normal",
    fontWeight: "400",
    fontSize: "20px",
    lineHeight: "130%",
}

const howItWorks = {
    width: "50%",
    marginLeft: "25%",
    marginTop: "30px",
    background: "#F4FEFF",
    borderRadius: "6px",
    fontFamily: 'Roboto',
    fontWeight: "400",
    fontSize: "16px",
    lineHeight: "130%",
    color: "#49454F",
    paddingLeft: "18px",
    paddingRight: "18px",
    paddingTop: "5px",
    paddingBottom: "5px",
}

const howItWorksSpan = {
    fontFamily: 'Inter',
    fontWeight: "700",
}

const inputBox = {
    padding: "10px",
    border: "1px solid black",
    width: "30%",
    marginLeft: "35%",
    borderRadius: "10px",
    marginTop: "10px",
    marginBottom: "10px",
}

const inputBoxInput = {
    width: "80%",
    height: "30px",
}

const inputBoxSelect = {
    width: "80%",
    height: "30px",
}


const inputBoxTitle = {
    display: "block",
    paddingBottom: "5px",
    fontSize: "12px",
    textAlign: "left",
    paddingLeft: "10%",
}

const commitButton = {
    backgroundColor: "#2D949A",
    color: "white",
    borderRadius: "100px",
    border: "0px",
    paddingLeft: "24px",
    paddingRight: "24px",
    paddingTop: "10px",
    paddingBottom: "10px",
}


return (
    <div style={signUp}>

        <div style={challengeTitle}>Wake up earlier</div>
        <div style={challengeSubTitle}>Commit to waking up every day at given time frame</div>

        <div style={howItWorks}>
            <span>How it works:</span>
            Once you commit to the challenge, you need to check in every morning on your active challenges page. Get
            your
            money back if you succeed!
        </div>

        <div style={inputBox}>
            <span style={inputBoxTitle}>Wake up period</span>
            <select style={inputBoxSelect} name="wakeupTime" id="wakeupTime">
                <option value="500">5:00AM - 6:00AM</option>
                <option value="600" selected>6:00AM - 7:00AM</option>
                <option value="700">7:00AM - 8:00AM</option>
            </select>
        </div>
        <div style={inputBox}>
            <span style={inputBoxTitle}>Duration</span>
            <select style={inputBoxSelect} name="duration" id="duration">
                <option value="10">10 days</option>
                <option value="20">20 days</option>
                <option value="30" selected>30 days</option>
            </select>
        </div>
        <div style={inputBox}>
            <span style={inputBoxTitle}>Permitted Skip Days</span>
            <select style={inputBoxSelect} name="skipDays" id="skipDays">
                <option value="0">0 days - hardcore option</option>
                <option value="3" selected>3 days</option>
                <option value="6">6 days</option>
            </select>
        </div>
        <div style={inputBox}>
            <span style={inputBoxTitle}>NEAR to stake</span>
            <input style={inputBoxInput} value="0.0" />
        </div>
        <button style={commitButton}>COMMIT TO CHALLENGE</button>
    </div>
)
