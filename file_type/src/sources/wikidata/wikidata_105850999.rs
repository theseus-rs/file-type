use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850999: FileFormat = FileFormat {
    id: 105_850_999,
    puid: "wikidata/105850999",
    name: "Terragen Object geometry",
    extensions: &["tgo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x45, 0x52, 0x52, 0x41, 0x47, 0x45, 0x4E, 0x47, 0x45, 0x4F, 0x4D, 0x45,
                    0x54, 0x52, 0x59, 0x44, 0x41, 0x54, 0x41, 0x1C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
