use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_383305: FileFormat = FileFormat {
    id: 383_305,
    puid: "wikidata/383305",
    name: "afio",
    extensions: &["af", "afio", "cpio"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
