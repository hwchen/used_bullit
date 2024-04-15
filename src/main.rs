use ansi_term::Color::Red;
use anyhow::Result;
use scraper::{Html, Selector};

fn main() -> Result<()> {
    let cities = &[
        // MA
        "boston",
        "worcester",
        "westernmass",
        "capecod",
        // NE states
        "nh",
        "vermont",
        "maine",
        // RI
        "providence",
        // CT
        "newlondon",
        "hartford",
        "newhaven",
        "nwct",
        // NY
        "albany",
        "hudsonvalley",
        "longisland",
        "nyc",
        // South of NY
        "philadelphia",
        "baltimore",
        // West Coast (just to test, not really feasible unless they're willin to ship)
        "seattle",
        //"portland",
        "sfbay",
    ];
    for city in cities {
        search_city(city)?;
    }

    Ok(())
}

fn search_city(city: &str) -> Result<()> {
    let body: String = ureq::get(&format!(
        "https://{city}.craigslist.org/search/bia?query=bullitt"
    ))
    .call()?
    .into_string()?;

    let document = Html::parse_document(&body);

    let result_info_sel = Selector::parse(".result-info").unwrap();
    let result_date_sel = Selector::parse(".result-date").unwrap();
    let result_price_sel = Selector::parse(".result-price").unwrap();
    let result_title_sel = Selector::parse(".result-title").unwrap();

    let city = Red.paint(city).to_string();
    println!("{city}");

    for result_info in document.select(&result_info_sel) {
        let result_title = result_info.select(&result_title_sel).next().unwrap();
        let title = result_title.text().collect::<Vec<_>>().join("");
        let link = result_title.value().attr("href").unwrap();

        let date = result_info
            .select(&result_date_sel)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        let price = result_info
            .select(&result_price_sel)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        println!("{price} {title} ({date}):\n{link}\n");
    }

    Ok(())
}
