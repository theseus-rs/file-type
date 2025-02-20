use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866965: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_965,
        source_type: SourceType::Wikidata,
        name: "KORG nanoKONTROL2 Editor data",
        extensions: &["nktrl2_data"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x30, 0x38, 0x34, 0x30, 0x53, 0x63, 0x6E, 0x53, 0x20, 0x00, 0x80, 0x01,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00,
                        0x53, 0x01, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x30, 0x38, 0x34, 0x30,
                        0x53, 0x63, 0x6E, 0x44, 0x20, 0x00, 0x53, 0x01, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x53, 0x01, 0x00, 0x00,
                        0xFF, 0xFF, 0xFF, 0xFF,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
