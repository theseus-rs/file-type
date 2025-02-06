use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58335773: FileFormat = FileFormat {
    id: 58_335_773,
    source_type: SourceType::Wikidata,
    name: "Verity Collection Stop List",
    extensions: &["stp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
