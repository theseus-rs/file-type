use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_383305: FileFormat = FileFormat {
    id: 383_305,
    source_type: SourceType::Wikidata,
    name: "afio",
    extensions: &["af", "afio", "cpio"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
