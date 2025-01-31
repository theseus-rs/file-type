use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858519: FileFormat = FileFormat {
    id: 105_858_519,
    puid: "wikidata/105858519",
    name: "ImageKnife Raw bitmap",
    extensions: &["raw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4B, 0x52, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
