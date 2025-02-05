use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28234649: FileFormat = FileFormat {
    id: 28_234_649,
    source_type: SourceType::Wikidata,
    name: "CCITT Group 3",
    extensions: &["g3"],
    media_types: &["image/g3fax"],
    signatures: &[],
    related_formats: &[],
};
