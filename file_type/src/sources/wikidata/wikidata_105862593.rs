use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862593: FileFormat = FileFormat {
    id: 105_862_593,
    source_type: SourceType::Wikidata,
    name: "Adventure Game Toolkit Messages",
    extensions: &["msg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x20, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
