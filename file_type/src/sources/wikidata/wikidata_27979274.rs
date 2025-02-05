use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979274: FileFormat = FileFormat {
    id: 27_979_274,
    source_type: SourceType::Wikidata,
    name: "TheDraw Library File",
    extensions: &["tdl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x18, 0x54, 0x68, 0x65, 0x44, 0x72, 0x61, 0x77, 0x20, 0x4C, 0x69, 0x62, 0x72,
                    0x61, 0x72, 0x79, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2E, 0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
