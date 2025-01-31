use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855738: FileFormat = FileFormat {
    id: 105_855_738,
    puid: "wikidata/105855738",
    name: "Meridian Driver",
    extensions: &["drv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x4D, 0x45, 0x52, 0x5F, 0x44, 0x52, 0x56, 0x25, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
