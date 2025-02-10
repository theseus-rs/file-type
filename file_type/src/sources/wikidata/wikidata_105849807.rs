use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849807: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_807,
        source_type: SourceType::Wikidata,
        name: "Calculux Indoor project",
        extensions: &["cin"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x61, 0x6C, 0x63, 0x75, 0x6C, 0x75, 0x78, 0x20, 0x49, 0x6E, 0x64,
                        0x6F, 0x6F, 0x72, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20,
                        0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
