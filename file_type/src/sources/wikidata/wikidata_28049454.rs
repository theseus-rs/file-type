use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049454: FileFormat = FileFormat {
    id: 28_049_454,
    puid: "wikidata/28049454",
    name: "DEGAS Elite Compressed, medium resolution",
    extensions: &["PC2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x80, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
