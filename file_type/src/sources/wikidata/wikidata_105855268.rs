use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855268: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_268,
        source_type: SourceType::Wikidata,
        name: "FastBack Plus Macro",
        extensions: &["fbm"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x28, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
