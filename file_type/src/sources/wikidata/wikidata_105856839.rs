use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856839: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_839,
        source_type: SourceType::Wikidata,
        name: "Myth: The Fallen Lords game data archive",
        extensions: &["gor"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x01, 0x00, 0x01, 0x70, 0x75, 0x62, 0x6C, 0x69, 0x63, 0x2E, 0x67,
                        0x6F, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
