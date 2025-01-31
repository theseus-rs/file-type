use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849988: FileFormat = FileFormat {
    id: 105_849_988,
    puid: "wikidata/105849988",
    name: "Classical Text Editor document (v9)",
    extensions: &["cte"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x43, 0x54, 0x45, 0x5F, 0x39, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
