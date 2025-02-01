use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856541: FileFormat = FileFormat {
    id: 105_856_541,
    puid: "wikidata/105856541",
    name: "AIM Extended Wavefunction",
    extensions: &["wfx"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3E, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
