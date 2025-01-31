use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858539: FileFormat = FileFormat {
    id: 105_858_539,
    puid: "wikidata/105858539",
    name: "PixArt bitmap (v2)",
    extensions: &["pix"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x58, 0x54, 0x00, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
