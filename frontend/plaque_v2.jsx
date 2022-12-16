
const challengeSummary = {
    fontSize: "24px",
    fontFamily: 'Roboto',
}

const mainBox = {
    width: "628px",
    borderRadius: "10px",
    backgroundColor: "#F6F6F6",
    padding: "20px",
    fontFamily: 'Inter',
    fontSize: "14px",
}

const challengeBox = {
    paddingTop: "20px",
    paddingBottom: "20px",
    paddingLeft: "10px",
    paddingRight: "10px",
}

const challengeBoxTitle = {
    fontSize: "16px",
    color: "#000000",
    fontFamily: 'Inter',
    fontWeight: "400",
    lineHeight: "19px",
}

const challengeBoxContents = {
    borderRadius: "5px",
    backgroundColor: "white",
    padding: "15px",
    textAlign: "center",
    marginTop: "20px",
    marginBottom: "20px",
    color: "#68717A",
}

const dot = {
    height: "20px",
    width: "20px",
    borderRadius: "50%",
    display: "inline-block",
}

const greenDot = {
    height: "20px",
    width: "20px",
    borderRadius: "50%",
    display: "inline-block",
    backgroundColor: "green",
}

const redDot = {
    height: "20px",
    width: "20px",
    borderRadius: "50%",
    display: "inline-block",
    backgroundColor: "red",
}

const greyDot = {
    height: "20px",
    width: "20px",
    borderRadius: "50%",
    display: "inline-block",
    backgroundColor: "grey",
}


const activeButton = {
    backgroundColor: "#2D949A",
    color: "white",
    borderRadius: "100px",
    border: "0px",
    paddingLeft: "24px",
    paddingRight: "24px",
    paddingTop: "10px",
    paddingBottom: "10px",
    marginTop: "10px",
}

const nearAmount = {
    fontSize: "16px",
    fontWeight: "900",
    color: "black",
}
const remainingBlock = {
    margin: "20px",
    backgroundColor: "#f4feff",
    borderRadius: "6px",
    color: "#68717a",
}

const dotHolder = {
    display: "inline-block",
}



// a == -1 (failed)
// a == 0 (unknown)
// a == 1 (success)
const renderDot = (day, a) => {
    let dotStyle = "";
    if (a == -1) {
        dotStyle = redDot;
    } else if (a == 0) {
        dotStyle = greyDot;
    } else {
        dotStyle = greenDot;
    }

    return (
        <div style={dotHolder}>
            <span style={dotStyle}></span><br />
            {day}
        </div>
    )
};

const renderDots = (totalDays, daysLeft, failedDaysList) => {
    let results = [];
    for (let day = 0; day < totalDays; day++) {
        if (failedDaysList.includes(day)) {
            results.push(renderDot(day + 1, -1));
        } else {
            if (day < (totalDays - daysLeft)) {
                results.push(renderDot(day + 1, 1));
            } else {
                results.push(renderDot(day + 1, 0));
            }
        }
    }
    return results
}

return (
    <div style={mainBox}>
        <div style={challengeSummary}> <img src="https://user-images.githubusercontent.com/91919554/208149479-340325cd-5151-4594-b7d6-ad91abe9d1ec.png"></img> Wake up earlier</div>
        <div style={challengeBox}>
            <div style={challengeBoxTitle}>Summary</div>
            <div style={challengeBoxContents}>
                You commited to waking up every day for <b>30 days</b> between <b>6:00-7:00</b>.
                <br />
                <button style={activeButton}>CHECK IN NOW</button>
            </div>
        </div>

        <div style={{ overflow: "hidden" }}>
            <div style={{ display: "inline-block", width: "30%", verticalAlign: "top" }}>
                <div style={challengeBox}>
                    <div style={challengeBoxTitle}>Current Rewards</div>
                    <div style={challengeBoxContents}>
                        <span style={nearAmount}>15 NEAR</span> <br /> 3 people
                    </div>
                </div>
            </div>
            <div style={{ display: "inline-block", width: "60%", verticalAlign: "top" }}>

                <div style={challengeBox} >
                    <div style={challengeBoxTitle}>Check-in progress</div>
                    <div style={challengeBoxContents}>

                        <div>
                            {renderDots(30, 5, [3, 4, 10])}
                        </div >
                        <div style={remainingBlock}>
                            <b>Remaining:</b> 26 days (2 skip days available)
                        </div>
                    </div >
                </div >
            </div>
        </div >
        <div style={{ display: "block" }}></div>

    </div >
)