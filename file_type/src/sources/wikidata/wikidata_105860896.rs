use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860896: FileFormat = FileFormat {
    id: 105_860_896,
    source_type: SourceType::Wikidata,
    name: "Rosegarden musical notation (RV21)",
    extensions: &["rose"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x21, 0x52, 0x6F, 0x73, 0x65, 0x67, 0x61, 0x72, 0x64, 0x65, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
