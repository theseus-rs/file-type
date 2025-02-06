use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125868657: FileFormat = FileFormat {
    id: 125_868_657,
    source_type: SourceType::Wikidata,
    name: "Common Data Format dotCDF 2.0-2.5",
    extensions: &["cdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
