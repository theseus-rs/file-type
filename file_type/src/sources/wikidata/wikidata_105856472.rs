use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856472: FileFormat = FileFormat {
    id: 105_856_472,
    puid: "wikidata/105856472",
    name: "GFA Raytrace project data (low-res)",
    extensions: &["wfl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x66, 0x6C, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
