// Let's say there are logs:
// System(requestid):
// - trace
// - error
// App(requestid):
// - trace
// - error
// - journal (human-readable summary)

// There's a prototype of something that can:
// - parse logs
// - filter
// -- by requestid
// -- by errors
// -- by account change (buy/sell)

use analysis::entities::Announcements;

// Data Model:
// - User (userid, name)
// - Items
// -- Item (assetid, name)
// -- Set (assetid, quantity)
// comment{-- Property (assetid, owner userid, quantity)}
// -- Supply Table (assetid per assetid, seller userid)
// -- Demand Table (assetid per assetid, buyer userid)
// - App Operation
// -- Journal
// --- Create user with authorized capital of 10 USD or more
// --- Delete user
// --- Register asset with liquidity of 50 USD
// --- Delete asset (all assets must be owned by the user)
// --- Deposit USD for userid (USD (aka US dollar) is an asset type)
// --- Withdraw USD for userid
// --- Buy asset
// --- Sell asset
// -- Trace
// --- Connect to the exchange
// --- Receive data from the exchange
// --- Local validation (preventing errors in the response)
// --- Send request to the exchange
// --- Receive a response from the exchange
// -- Error
// --- No asset
// --- System error
// - System operation
// -- Trace
// --- Send request
// --- Receive a response
// -- Error
// --- No network
// --- Access denied
fn main() {
    println!("Placeholder для экспериментов с cli");

    let parsing_demo =
        r#"[UserBackets{"user_id":"Bob","backets":[Backet{"asset_id":"milk","count":3,},],},]"#;
    let announcements = analysis::entities::just_parse::<Announcements>(parsing_demo).unwrap();
    println!("demo-parsed: {:?}", announcements);

    let args = std::env::args().collect::<Vec<_>>();

    let filename = args.get(1);
    if let Some(filename) = filename {
        println!(
            "Trying opening file '{}' from directory '{}'",
            filename,
            std::env::current_dir()
                .expect("current directory not found")
                .to_string_lossy()
        );
        let file = std::fs::File::open(filename).expect("file not found");

        let logs = analysis::read_log(file, analysis::ReadMode::All, vec![]);
        println!("got logs:");
        logs.iter().for_each(|parsed| println!("  {:?}", parsed));
    }
}
