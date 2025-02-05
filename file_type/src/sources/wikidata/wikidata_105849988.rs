use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849988: FileFormat = FileFormat {
    id: 105_849_988,
    source_type: SourceType::Wikidata,
    name: "Classical Text Editor document (v9)",
    extensions: &["cte"],
    media_types: &["text/xml"],
    signatures: &[Signature {
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
