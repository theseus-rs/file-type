use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856903: FileFormat = FileFormat {
    id: 105_856_903,
    source_type: SourceType::Wikidata,
    name: "GCG Sequence Chemical file",
    extensions: &["gcg"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x45, 0x41, 0x54, 0x55, 0x52, 0x45, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
