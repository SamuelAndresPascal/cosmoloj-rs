trait UnitConverter {
  fn scale(&self) -> f64;
  fn offset(&self) -> f64;
  fn inverse(&self) -> dyn UnitConverter;
  fn linear(&self) -> dyn UnitConverter;
  fn linear_pow(&self, power: f64) -> dyn UnitConverter;
  fn convert(&self, value: f64) -> f64;
  fn concatenate(&self, converter: dyn UnitConverter) -> dyn UnitConverter;
}

trait Factor {
  fn dim(&self) -> dyn Unit;
  fn numerator(&self) -> i32;
  fn denominator(&self) -> i32;
  fn power(&self) -> f64;
}

trait Unit {
  fn get_converter_to(&self, target: dyn Unit) -> dyn UnitConverter;
  fn to_base(&self) -> dyn UnitConverter;
  fn shift(&self, value: f64) -> dyn Unit;
  fn scale_multiply(&self, value: f64) -> dyn Unit;
  fn scale_divide(&self, value: f64) -> dyn Unit;
  fn factor(&self, numerator: i32, denominator: i32) -> dyn Factor;
}

trait FundamentalUnit {
}

trait TransformedUnit {
  fn to_reference(&self) -> dyn UnitConverter;
  fn reference(&self) -> dyn Unit;
}

trait DerivedUnit {
  fn definition(&self) -> Vec<&dyn Factor>;
}
