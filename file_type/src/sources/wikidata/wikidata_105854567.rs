use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854567: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_567,
        source_type: SourceType::Wikidata,
        name: "freeCAD assembly",
        extensions: &["asm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4F, 0x53, 0x53, 0x20, 0x37, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
