use broker_statement::ib::IbStatementParser;
use core::EmptyResult;
use currency::CashAssets;
use currency::converter::CurrencyConverter;
use db;

pub mod performance;

pub fn analyse(database: db::Connection, broker_statement_path: &str) -> EmptyResult {
    let statement = IbStatementParser::new().parse(broker_statement_path).map_err(|e| format!(
        "Error while reading {:?} broker statement: {}", broker_statement_path, e))?;
    debug!("{:#?}", statement);

    let converter = CurrencyConverter::new(database);
    let total_value = CashAssets::new_from_cash(statement.period.1, statement.total_value);

    // FIXME: Deprecate
    for currency in ["USD", "RUB"].iter() {
        let interest = performance::get_average_profit(
            &statement.deposits, total_value, *currency, &converter)?;

        println!("{}: {}", currency, interest * dec!(100));
    }

    println!("Average rate of return from cash investments:");

    for currency in ["USD", "RUB"].iter() {
        let interest = performance::get_average_rate_of_return(
            &statement.deposits, total_value, *currency, &converter)?;

        println!("{}: {}", currency, interest);
    }

    Ok(())
}