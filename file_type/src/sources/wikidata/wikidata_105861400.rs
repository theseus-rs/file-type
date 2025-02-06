use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861400: FileFormat = FileFormat {
    id: 105_861_400,
    source_type: SourceType::Wikidata,
    name: "LucasArts game data archive",
    extensions: &["lab"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x41, 0x42, 0x4E, 0x00, 0x00, 0x01, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
