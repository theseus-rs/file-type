use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852820: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_820,
        source_type: SourceType::Wikidata,
        name: "Qucs schematic (v0.0.x)",
        extensions: &["sch"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x51, 0x75, 0x63, 0x73, 0x20, 0x53, 0x63, 0x68, 0x65, 0x6D, 0x61,
                        0x74, 0x69, 0x63, 0x20, 0x30, 0x2E, 0x30, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
