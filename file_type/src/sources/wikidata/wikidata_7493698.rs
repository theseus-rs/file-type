use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7493698: FileFormat = FileFormat {
    id: 7_493_698,
    source_type: SourceType::Wikidata,
    name: "Shell Scrap Object File",
    extensions: &["shs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
