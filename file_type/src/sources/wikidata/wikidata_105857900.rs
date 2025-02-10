use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857900: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_900,
        source_type: SourceType::Wikidata,
        name: "InterFont font (v1.0)",
        extensions: &["if"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x6E, 0x74, 0x65, 0x72, 0x46, 0x6F, 0x6E, 0x74, 0x20, 0x31, 0x2E,
                        0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
