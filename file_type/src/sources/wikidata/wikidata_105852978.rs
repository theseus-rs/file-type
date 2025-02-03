use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852978: FileFormat = FileFormat {
    id: 105_852_978,
    source_type: SourceType::Wikidata,
    name: "Session Description Protocol (with rem)",
    extensions: &["sdp"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
