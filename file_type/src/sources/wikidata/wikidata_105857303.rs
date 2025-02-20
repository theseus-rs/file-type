use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857303: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_303,
        source_type: SourceType::Wikidata,
        name: "Free Hex Editor Neo Settings",
        extensions: &["hexset"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD4, 0x40, 0xAE, 0x72, 0x23, 0xBD, 0xE2, 0x4A, 0xA6, 0x78, 0xD0, 0x4F,
                        0x75, 0x94, 0x7E, 0xBA,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
