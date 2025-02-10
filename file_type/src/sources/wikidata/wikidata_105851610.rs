use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851610: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_610,
        source_type: SourceType::Wikidata,
        name: "Torque module definition (XML)",
        extensions: &["taml"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x44, 0x65, 0x66, 0x69, 0x6E,
                        0x69, 0x74, 0x69, 0x6F, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
