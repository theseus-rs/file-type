use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61777776: FileFormat = FileFormat {
    id: 61_777_776,
    source_type: SourceType::Wikidata,
    name: "Play SID Audio, version 1",
    extensions: &["dxr", "psid"],
    media_types: &["audio/prs.sid"],
    internal_signatures: &[],
    related_formats: &[],
};
