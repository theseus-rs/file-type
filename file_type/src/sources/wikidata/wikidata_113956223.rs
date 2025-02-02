use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113956223: FileFormat = FileFormat {
    id: 113_956_223,
    source_type: SourceType::Wikidata,
    name: "Software-Independent Archiving of Relational Databases, version 2.1",
    extensions: &["siard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
