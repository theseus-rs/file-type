use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864905: FileFormat = FileFormat {
    id: 105_864_905,
    puid: "wikidata/105864905",
    name: "PocketBook Theme",
    extensions: &["pbt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x63, 0x6B, 0x65, 0x74, 0x42, 0x6F, 0x6F, 0x6B, 0x54, 0x68, 0x65,
                    0x6D, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
