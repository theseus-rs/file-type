use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122072541: FileFormat = FileFormat {
    id: 122_072_541,
    source_type: SourceType::Wikidata,
    name: "Rhapsody File",
    extensions: &["rhp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
