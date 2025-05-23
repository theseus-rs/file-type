use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849847: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_847,
        source_type: SourceType::Wikidata,
        name: "Syslinux COM32 module (v2)",
        extensions: &["c32"],
        media_types: &["application/x-c32-comboot-syslinux-exec"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB8, 0xFF, 0x4C, 0xCD, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
