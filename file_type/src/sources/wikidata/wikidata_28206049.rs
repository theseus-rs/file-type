use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206049: FileFormat = FileFormat {
    id: 28_206_049,
    puid: "wikidata/28206049",
    name: "ERDAS Imagine IMG",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
