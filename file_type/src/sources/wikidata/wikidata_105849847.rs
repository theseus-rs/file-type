use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849847: FileFormat = FileFormat {
    id: 105_849_847,
    puid: "wikidata/105849847",
    name: "Syslinux COM32 module (v2)",
    extensions: &["c32"],
    media_types: &["application/x-c32-comboot-syslinux-exec"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB8, 0xFF, 0x4C, 0xCD, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
