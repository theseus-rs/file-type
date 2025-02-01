use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851207: FileFormat = FileFormat {
    id: 105_851_207,
    puid: "wikidata/105851207",
    name: "TruePaint Animation",
    extensions: &["tpa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x50, 0x41, 0x00, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
