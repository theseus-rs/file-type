use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_11224899: FileFormat = FileFormat {
    id: 11_224_899,
    source_type: SourceType::Wikidata,
    name: "ish",
    extensions: &["ish"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
