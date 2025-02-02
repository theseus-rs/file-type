use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61777964: FileFormat = FileFormat {
    id: 61_777_964,
    source_type: SourceType::Wikidata,
    name: "Play SID Audio, version 2",
    extensions: &["psid", "sid"],
    media_types: &["audio/prs.sid"],
    internal_signatures: &[],
    related_formats: &[],
};
