use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967221: FileFormat = FileFormat {
    id: 27_967_221,
    source_type: SourceType::Wikidata,
    name: "Soundtracker v2.6/Ice Tracker module",
    extensions: &["st26"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4E, 0x54, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
