use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967114: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_114,
        source_type: SourceType::Wikidata,
        name: "Arkos Tracker",
        extensions: &["aks"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x54, 0x31, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
