use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_61901754: FileType = FileType {
    file_format: &FileFormat {
        id: 61_901_754,
        source_type: SourceType::Wikidata,
        name: "EndNote Filter File",
        extensions: &["enf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x4E, 0x44, 0x4E, 0x45, 0x4C, 0x32, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
