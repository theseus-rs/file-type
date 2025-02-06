use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852978: FileFormat = FileFormat {
    id: 105_852_978,
    source_type: SourceType::Wikidata,
    name: "Session Description Protocol (with rem)",
    extensions: &["sdp"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
