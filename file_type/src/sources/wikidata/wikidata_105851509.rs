use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851509: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_509,
        source_type: SourceType::Wikidata,
        name: "Terraria Mod",
        extensions: &["tmod"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x4D, 0x4F, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
