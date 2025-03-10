use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852149: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_149,
        source_type: SourceType::Wikidata,
        name: "Surreal Software game data container",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x52, 0x53, 0x43, 0x01, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
