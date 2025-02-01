use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864995: FileFormat = FileFormat {
    id: 105_864_995,
    puid: "wikidata/105864995",
    name: "Professional Music Driver PZI samples pack (v0)",
    extensions: &["pzi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x5A, 0x49, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
