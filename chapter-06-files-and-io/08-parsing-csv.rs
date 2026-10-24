// Chapter 6, Lesson 8: Reading and writing CSV data by hand
// Run: cargo run --example ch06-08

use std::fs;
use std::io;

#[derive(Debug)]
struct Sale {
    date: String,
    product: String,
    quantity: u32,
    price: f64,
}

fn parse_sale(line: &str, line_no: usize) -> Result<Sale, String> {
    let fields: Vec<&str> = line.split(',').map(|f| f.trim()).collect();
    let [date, product, quantity, price] = fields[..] else {
        return Err(format!("line {line_no}: expected 4 fields, found {}", fields.len()));
    };
    Ok(Sale {
        date: date.to_string(),
        product: product.to_string(),
        quantity: quantity.parse().map_err(|_| format!("line {line_no}: bad quantity '{quantity}'"))?,
        price: price.parse().map_err(|_| format!("line {line_no}: bad price '{price}'"))?,
    })
}

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir();
    let input = dir.join("rust199_sales.csv");
    fs::write(
        &input,
        "date,product,quantity,price\n\
         2026-10-01,Notebook,3,45.00\n\
         2026-10-01,Pen,10,12.50\n\
         2026-10-02,Notebook,1,45.00\n\
         2026-10-02,Bag,one,799.00\n\
         2026-10-03,Pen,4,12.50\n",
    )?;

    let text = fs::read_to_string(&input)?;
    let mut sales = Vec::new();
    // skip(1) skips the header row
    for (i, line) in text.lines().enumerate().skip(1) {
        match parse_sale(line, i + 1) {
            Ok(sale) => sales.push(sale),
            Err(e) => println!("Skipping: {e}"),
        }
    }
    println!("Loaded {} sales", sales.len());

    // Summarise revenue per product.
    let mut summary: Vec<(String, u32, f64)> = Vec::new();
    for s in &sales {
        match summary.iter_mut().find(|(p, _, _)| *p == s.product) {
            Some(row) => {
                row.1 += s.quantity;
                row.2 += s.quantity as f64 * s.price;
            }
            None => summary.push((s.product.clone(), s.quantity, s.quantity as f64 * s.price)),
        }
    }

    // Write the summary as a new CSV file.
    let output = dir.join("rust199_summary.csv");
    let mut out = String::from("product,units,revenue\n");
    for (product, units, revenue) in &summary {
        out.push_str(&format!("{product},{units},{revenue:.2}\n"));
    }
    fs::write(&output, &out)?;
    print!("{out}");
    println!("First sale was on {}", sales[0].date);

    fs::remove_file(input)?;
    fs::remove_file(output)?;
    Ok(())
}
