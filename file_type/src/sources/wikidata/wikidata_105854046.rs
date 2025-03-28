use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854046: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_046,
        source_type: SourceType::Wikidata,
        name: "PSION Application/Image executable",
        extensions: &["app", "img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x6D, 0x61, 0x67, 0x65, 0x46, 0x69, 0x6C, 0x65, 0x54, 0x79, 0x70,
                        0x65, 0x2A, 0x2A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
