use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858236: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_236,
        source_type: SourceType::Wikidata,
        name: "EstImage DataBase",
        extensions: &["edb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCC, 0x33, 0x00, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
