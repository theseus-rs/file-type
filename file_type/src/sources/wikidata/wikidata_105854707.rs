use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854707: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_707,
        source_type: SourceType::Wikidata,
        name: "Windows Policy Administrative Template (Unicode)",
        extensions: &["adm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x43, 0x00, 0x4C, 0x00, 0x41, 0x00, 0x53, 0x00, 0x53, 0x00,
                        0x20, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
