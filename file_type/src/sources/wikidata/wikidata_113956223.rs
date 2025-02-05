use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113956223: FileFormat = FileFormat {
    id: 113_956_223,
    source_type: SourceType::Wikidata,
    name: "Software-Independent Archiving of Relational Databases, version 2.1",
    extensions: &["siard"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
