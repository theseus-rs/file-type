use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125868958: FileFormat = FileFormat {
    id: 125_868_958,
    puid: "wikidata/125868958",
    name: "Common Data Format dotCDF 3.x",
    extensions: &["cdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
