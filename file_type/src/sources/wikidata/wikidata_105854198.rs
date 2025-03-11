use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854198: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_198,
        source_type: SourceType::Wikidata,
        name: "RS compressed archive",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x53, 0x56, 0x4B, 0x44, 0x41, 0x54, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
