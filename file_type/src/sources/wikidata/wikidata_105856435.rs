use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856435: FileFormat = FileFormat {
    id: 105_856_435,
    puid: "wikidata/105856435",
    name: "GFA Raytrace Animation (hi-res)",
    extensions: &["wah"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x61, 0x68, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
