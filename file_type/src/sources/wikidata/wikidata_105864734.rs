use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864734: FileFormat = FileFormat {
    id: 105_864_734,
    puid: "wikidata/105864734",
    name: "Professional Music Driver PVI samples pack (v1)",
    extensions: &["pvi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x56, 0x49, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
