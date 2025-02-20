use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851406: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_406,
        source_type: SourceType::Wikidata,
        name: "Torque animation asset (XML)",
        extensions: &["taml"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x41, 0x6E, 0x69, 0x6D, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x41, 0x73,
                        0x73, 0x65, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
