use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855227: FileFormat = FileFormat {
    id: 105_855_227,
    puid: "wikidata/105855227",
    name: "Freeze compressed data (v1.x)",
    extensions: &["f"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x9E])],
            },
        }],
    }],
    related_formats: &[],
};
