use polars::df;
use polars_core::frame::DataFrame;
use lancedb::{connect, Result, Table as LanceDbTable};
use polars_lazy::frame::LazyFrame;


#[allow(dead_code)]
fn from_polars(data: DataFrame ) {
    //TODO: Add code here

}

#[allow(dead_code)]
fn from_polars_lazy(data: DataFrame) {
    //TODO: Add code here
}

//TODO: Add tests here later


//TODO: Convert from Polars DF to LanceDB table
fn main() {
    //1. Initialize a Polars DF 
    let test_vec = df! [
        "names" => ["one", "two"],
        "values" => [0.2, 0.3],
        "values_nulls" => [Some(0.2), Some(0.3)]
        ];

    println!("{:?}", test_vec);



    //2. Convert to LanceDB Table 


    //3. Display the new table
    
    print!("Hello World!");
}
