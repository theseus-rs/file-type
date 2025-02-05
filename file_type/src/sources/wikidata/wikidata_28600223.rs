use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600223: FileFormat = FileFormat {
    id: 28_600_223,
    source_type: SourceType::Wikidata,
    name: "APT",
    extensions: &["apt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
