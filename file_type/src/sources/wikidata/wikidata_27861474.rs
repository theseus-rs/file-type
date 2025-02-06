use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27861474: FileFormat = FileFormat {
    id: 27_861_474,
    source_type: SourceType::Wikidata,
    name: "Software Independent Archiving of Relational Databases, version 2.0",
    extensions: &["siard"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
