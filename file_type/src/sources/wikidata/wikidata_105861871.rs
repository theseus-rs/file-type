use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861871: FileFormat = FileFormat {
    id: 105_861_871,
    puid: "wikidata/105861871",
    name: "Money Matters data (v3)",
    extensions: &["mm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x4D, 0x44, 0x4A, 0x4D, 0x30, 0x30, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
