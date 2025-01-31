use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58077776: FileFormat = FileFormat {
    id: 58_077_776,
    puid: "wikidata/58077776",
    name: "AbiWord Document Template",
    extensions: &["awt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
