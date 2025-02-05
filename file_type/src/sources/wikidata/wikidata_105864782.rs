use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864782: FileFormat = FileFormat {
    id: 105_864_782,
    source_type: SourceType::Wikidata,
    name: "No Man's Sky game data",
    extensions: &["pak"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x41, 0x52, 0x00, 0x01, 0x00, 0x04, 0x7A, 0x6C, 0x69, 0x62, 0x00,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
