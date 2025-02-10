use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_34743342: FileType = FileType {
    file_format: &FileFormat {
        id: 34_743_342,
        source_type: SourceType::Wikidata,
        name: "Softimage Scene",
        extensions: &["scn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
