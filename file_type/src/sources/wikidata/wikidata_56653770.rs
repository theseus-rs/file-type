use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_56653770: FileType = FileType {
    file_format: &FileFormat {
        id: 56_653_770,
        source_type: SourceType::Wikidata,
        name: "Final Cut Pro Project",
        extensions: &["fcp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA2, 0x4B, 0x65, 0x79, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
