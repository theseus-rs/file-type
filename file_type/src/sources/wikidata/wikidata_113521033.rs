use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113521033: FileFormat = FileFormat {
    id: 113_521_033,
    source_type: SourceType::Wikidata,
    name: "BIM Metadata File",
    extensions: &["bim"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
