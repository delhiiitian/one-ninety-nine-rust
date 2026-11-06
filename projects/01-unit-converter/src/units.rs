// The data table: every unit the converter knows about.
//
// Each unit is converted to a "base" unit of its kind first, then from
// the base to the target. That way N units need N table rows, not N*N
// conversion formulas.
//
//   base_value = value * factor + offset
//   value      = (base_value - offset) / factor
//
// Base units: metre (length), kilogram (mass), Celsius (temperature),
// litre (volume).

/// (symbol, full name, kind, factor, offset)
pub type Unit = (&'static str, &'static str, &'static str, f64, f64);

pub const UNITS: [Unit; 17] = [
    // length, base = metre
    ("mm", "millimetre", "length", 0.001, 0.0),
    ("cm", "centimetre", "length", 0.01, 0.0),
    ("m", "metre", "length", 1.0, 0.0),
    ("km", "kilometre", "length", 1000.0, 0.0),
    ("in", "inch", "length", 0.0254, 0.0),
    ("ft", "foot", "length", 0.3048, 0.0),
    ("mi", "mile", "length", 1609.344, 0.0),
    // mass, base = kilogram
    ("g", "gram", "mass", 0.001, 0.0),
    ("kg", "kilogram", "mass", 1.0, 0.0),
    ("lb", "pound", "mass", 0.453_592_37, 0.0),
    ("oz", "ounce", "mass", 0.028_349_523_125, 0.0),
    // temperature, base = Celsius
    ("c", "Celsius", "temperature", 1.0, 0.0),
    ("f", "Fahrenheit", "temperature", 5.0 / 9.0, -32.0 * 5.0 / 9.0),
    ("k", "Kelvin", "temperature", 1.0, -273.15),
    // volume, base = litre
    ("ml", "millilitre", "volume", 0.001, 0.0),
    ("l", "litre", "volume", 1.0, 0.0),
    ("gal", "US gallon", "volume", 3.785_411_784, 0.0),
];

/// Finds a unit by its symbol, ignoring upper/lower case.
pub fn find(symbol: &str) -> Option<Unit> {
    let wanted = symbol.to_lowercase();
    for unit in UNITS {
        if unit.0 == wanted {
            return Some(unit);
        }
    }
    None
}

/// Converts `value` from one unit to another of the same kind.
pub fn convert(value: f64, from: Unit, to: Unit) -> Result<f64, String> {
    if from.2 != to.2 {
        return Err(format!(
            "cannot convert {} ({}) to {} ({})",
            from.1, from.2, to.1, to.2
        ));
    }
    let base = value * from.3 + from.4;
    Ok((base - to.4) / to.3)
}
