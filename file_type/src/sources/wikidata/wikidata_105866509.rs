use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866509: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_509,
        source_type: SourceType::Wikidata,
        name: "Packed Bohemia Object game data archive",
        extensions: &["pbo"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x73, 0x72, 0x65, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
