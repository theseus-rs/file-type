use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852975: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_975,
        source_type: SourceType::Wikidata,
        name: "SBF game data container",
        extensions: &["sbf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x42, 0x46, 0x30, 0x00, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
