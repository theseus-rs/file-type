use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850563: FileFormat = FileFormat {
    id: 105_850_563,
    puid: "wikidata/105850563",
    name: "CATIA Drawing (v5 r20)",
    extensions: &["catdrawing"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x35, 0x5F, 0x43, 0x46, 0x56, 0x32, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
