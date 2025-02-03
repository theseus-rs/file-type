use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_209054: FileFormat = FileFormat {
    id: 209_054,
    source_type: SourceType::Wikidata,
    name: "Audio Video Interleave",
    extensions: &["avi"],
    media_types: &["video/vnd.avi"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x56, 0x49, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
