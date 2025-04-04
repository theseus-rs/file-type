use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860355: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_355,
        source_type: SourceType::Wikidata,
        name: "Farallon Replica document",
        extensions: &["rpl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x61, 0x72, 0x61, 0x6C, 0x6C, 0x6F, 0x6E, 0x20, 0x52, 0x65, 0x70,
                        0x6C, 0x69, 0x63, 0x61, 0x20, 0x28, 0x54, 0x4D, 0x29, 0x20, 0x20, 0x20,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
