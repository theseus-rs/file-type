use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866436: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_436,
        source_type: SourceType::Wikidata,
        name: "Panzerkrieg for Windows Scenario",
        extensions: &["pks"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46, 0x57, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
