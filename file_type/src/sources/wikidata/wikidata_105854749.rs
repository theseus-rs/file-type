use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854749: FileFormat = FileFormat {
    id: 105_854_749,
    puid: "wikidata/105854749",
    name: "ARRIRAW image",
    extensions: &["ari"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x52, 0x52, 0x49, 0x12, 0x34, 0x56, 0x78,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
