use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854257: FileFormat = FileFormat {
    id: 105_854_257,
    puid: "wikidata/105854257",
    name: "Musepack encoded audio (SV7.0)",
    extensions: &["mpc"],
    media_types: &["audio/musepack"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x2B, 0x07])],
            },
        }],
    }],
    related_formats: &[],
};
