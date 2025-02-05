use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130405004: FileFormat = FileFormat {
    id: 130_405_004,
    source_type: SourceType::Wikidata,
    name: "Org file",
    extensions: &["org"],
    media_types: &["text/org"],
    signatures: &[],
    related_formats: &[],
};
