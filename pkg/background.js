console.log(this)

// Function to do something Periodically
async function HelloBackground() {
    console.log("Hello from Background")
}

// Function to set up the alarm for 10 seconds
function setupAlarm() {
    chrome.alarms.create("HelloAlarm", { delayInMinutes: 0.1667 }) // 10 seconds = 0.1667 minutes
}


// Listen for the alarm event
chrome.alarms.onAlarm.addListener((alarm) => {
    if (alarm.name === "HelloAlarm") {
        HelloBackground().then(setupAlarm())
    }
})

// Optional: Initialize the background worker
chrome.runtime.onInstalled.addListener(() => {
    console.log("Background worker installed and running.")
    setupAlarm()
})
