use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851391: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_391,
        source_type: SourceType::Wikidata,
        name: "Torque sprite asset (XML)",
        extensions: &["taml"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x53, 0x70, 0x72, 0x69, 0x74, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
