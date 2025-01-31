use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125868657: FileFormat = FileFormat {
    id: 125_868_657,
    puid: "wikidata/125868657",
    name: "Common Data Format dotCDF 2.0-2.5",
    extensions: &["cdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
