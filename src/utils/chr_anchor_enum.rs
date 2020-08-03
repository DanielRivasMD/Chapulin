
////////////////////////////////////////////////////////////////////////////////////////////////////

/// |---------------------------------------------|---------------------------------------------|
/// | C anchor [unmpped] (position) {orientation} | ME anchor [mapped] (position) {orientation} |
/// |---------------------------------------------|---------------------------------------------|
/// | complete [*] (mate) {inwards}               | complete [100M] (ME limit) {outwards}       |
/// | complete [*] (mate) {inwards}               | partial [50S50M] (ME limit) {outwards}      |
/// | partial [50S50M] (ME limit) {inwards}       | complete [100M]  (ME limit) {outwards}      |
/// | partial [50S50M] (ME limit) {inwards}       | partial [50S50M]  (ME limit) {outwards}     |
/// |---------------------------------------------|---------------------------------------------|
#[derive(Debug, new)]
pub enum ChrAnchorEnum {
  Read1,
  Read2,
  None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
