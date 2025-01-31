use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852129: FileFormat = FileFormat {
    id: 105_852_129,
    puid: "wikidata/105852129",
    name: "GFA Raytrace compressed Animation (low-res)",
    extensions: &["sal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x61, 0x6C, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
