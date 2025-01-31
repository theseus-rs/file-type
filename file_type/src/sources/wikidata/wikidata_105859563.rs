use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859563: FileFormat = FileFormat {
    id: 105_859_563,
    puid: "wikidata/105859563",
    name: "Samplitude Virtual Project (old ver)",
    extensions: &["vip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x45, 0x4B, 0x44, 0x50, 0x4C, 0x41, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
