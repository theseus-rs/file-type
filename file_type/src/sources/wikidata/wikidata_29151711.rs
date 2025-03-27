use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29151711: FileType = FileType {
    file_format: &FileFormat {
        id: 29_151_711,
        source_type: SourceType::Wikidata,
        name: "RK",
        extensions: &["rk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xAA, 0x04, 0x00, 0x00, 0x00, 0x00, 0xAD, 0xBB, 0xDA, 0x04, 0x00, 0x00,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
