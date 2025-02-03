use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115037903: FileFormat = FileFormat {
    id: 115_037_903,
    source_type: SourceType::Wikidata,
    name: "Software-Independent Archiving of Relational Databases 2.2",
    extensions: &["siard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
