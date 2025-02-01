use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855816: FileFormat = FileFormat {
    id: 105_855_816,
    puid: "wikidata/105855816",
    name: "Oric MFM disk image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x46, 0x4D, 0x5F, 0x44, 0x49, 0x53, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
