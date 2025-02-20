use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856327: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_327,
        source_type: SourceType::Wikidata,
        name: "RomCenter format",
        extensions: &["dat"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x43, 0x52, 0x45, 0x44, 0x49, 0x54, 0x53, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
