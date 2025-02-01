use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851468: FileFormat = FileFormat {
    id: 105_851_468,
    puid: "wikidata/105851468",
    name: "TI-Nspire CAS OS image",
    extensions: &["tnc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x49, 0x2D, 0x4E, 0x73, 0x70, 0x69, 0x72, 0x65, 0x2E, 0x74, 0x6E, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
