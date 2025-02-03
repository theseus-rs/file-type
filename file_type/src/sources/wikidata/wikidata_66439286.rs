use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66439286: FileFormat = FileFormat {
    id: 66_439_286,
    source_type: SourceType::Wikidata,
    name: "DisplayWrite Document file format, version 5",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
