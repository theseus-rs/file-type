use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851378: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_378,
        source_type: SourceType::Wikidata,
        name: "GrandMA1 fixture",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5F, 0x46, 0x49, 0x58, 0x54, 0x55, 0x52, 0x45, 0x54, 0x59, 0x50, 0x45,
                        0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
