use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_383305: FileFormat = FileFormat {
    id: 383_305,
    source_type: SourceType::Wikidata,
    name: "afio",
    extensions: &["af", "afio", "cpio"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
