use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866325: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_325,
        source_type: SourceType::Wikidata,
        name: "smARTWORK Printed Circuit Board project",
        extensions: &["pcb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x77, 0x62, 0x63, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
