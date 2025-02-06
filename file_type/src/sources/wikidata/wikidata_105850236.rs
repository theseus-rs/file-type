use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850236: FileFormat = FileFormat {
    id: 105_850_236,
    source_type: SourceType::Wikidata,
    name: "Syslinux COM32 module (v4)",
    extensions: &["c32"],
    media_types: &["application/x-c32-comboot-syslinux-exec"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB8, 0xFE, 0x4C, 0xCD, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
