use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851971: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_971,
        source_type: SourceType::Wikidata,
        name: "Independence War 2: Edge of Chaos save game",
        extensions: &["sav"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x07, 0x56, 0x47, 0x53, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
