use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858424: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_424,
        source_type: SourceType::Wikidata,
        name: "Solace ENTER hex format (abbreviated)",
        extensions: &["ent"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4E, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
