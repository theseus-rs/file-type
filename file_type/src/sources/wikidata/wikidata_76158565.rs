use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76158565: FileFormat = FileFormat {
    id: 76_158_565,
    puid: "wikidata/76158565",
    name: "MegaPaint Vector symbols Library",
    extensions: &["vlb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x56, 0x4C, 0x42, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
