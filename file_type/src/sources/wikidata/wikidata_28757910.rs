use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757910: FileFormat = FileFormat {
    id: 28_757_910,
    source_type: SourceType::Wikidata,
    name: "Google Document",
    extensions: &["gdoc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
