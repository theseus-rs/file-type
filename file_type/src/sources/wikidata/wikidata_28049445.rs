use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049445: FileFormat = FileFormat {
    id: 28_049_445,
    puid: "wikidata/28049445",
    name: "DEGAS Elite Compressed, low resolution",
    extensions: &["PC1"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x80, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
