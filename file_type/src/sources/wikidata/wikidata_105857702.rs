use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857702: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_702,
        source_type: SourceType::Wikidata,
        name: "Multibit Bitcoin wallet info",
        extensions: &["info"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6D, 0x75, 0x6C, 0x74, 0x69, 0x42, 0x69, 0x74, 0x2E, 0x69, 0x6E, 0x66,
                        0x6F, 0x2C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
