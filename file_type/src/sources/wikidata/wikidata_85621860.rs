use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_85621860: FileFormat = FileFormat {
    id: 85_621_860,
    source_type: SourceType::Wikidata,
    name: "PFS:First Choice Database",
    extensions: &["fol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
