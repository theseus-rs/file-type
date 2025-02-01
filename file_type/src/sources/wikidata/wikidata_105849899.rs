use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849899: FileFormat = FileFormat {
    id: 105_849_899,
    puid: "wikidata/105849899",
    name: "Citect Trend History data (v2)",
    extensions: &["001"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x49, 0x54, 0x45, 0x43, 0x54, 0x0A, 0x0D, 0x56, 0x45, 0x52, 0x53, 0x49,
                    0x4F, 0x4E, 0x20, 0x32, 0x0A, 0x0D, 0x54, 0x52, 0x45, 0x4E, 0x44, 0x20, 0x48,
                    0x49, 0x53, 0x54, 0x4F, 0x52, 0x59, 0x0A, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
