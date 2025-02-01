use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855721: FileFormat = FileFormat {
    id: 105_855_721,
    puid: "wikidata/105855721",
    name: "UCDOS Overlay",
    extensions: &["ovr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x43, 0x44, 0x4F, 0x53, 0x20, 0x4F, 0x56, 0x52, 0x20, 0x46, 0x49, 0x4C,
                    0x45, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
