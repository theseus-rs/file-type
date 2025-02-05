use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861960: FileFormat = FileFormat {
    id: 105_861_960,
    source_type: SourceType::Wikidata,
    name: "Quake 3 Model (newer)",
    extensions: &["md4"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x44, 0x50, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
