use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854809: FileFormat = FileFormat {
    id: 105_854_809,
    source_type: SourceType::Wikidata,
    name: "mp3HD audio",
    extensions: &["mp3"],
    media_types: &["audio/mpeg3"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
