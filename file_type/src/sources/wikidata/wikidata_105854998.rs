use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854998: FileFormat = FileFormat {
    id: 105_854_998,
    puid: "wikidata/105854998",
    name: "Musepack encoded audio (SV8)",
    extensions: &["mpc"],
    media_types: &["audio/musepack"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x43, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
