use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855184: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_184,
        source_type: SourceType::Wikidata,
        name: "Firebird database",
        extensions: &["fdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x00, 0x39, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
