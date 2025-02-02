use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66141873: FileFormat = FileFormat {
    id: 66_141_873,
    source_type: SourceType::Wikidata,
    name: "MDE file format",
    extensions: &["mde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
