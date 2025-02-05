use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857618: FileFormat = FileFormat {
    id: 105_857_618,
    source_type: SourceType::Wikidata,
    name: "PLS-CADD Import info",
    extensions: &["imp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x59, 0x50, 0x45, 0x3D, 0x27, 0x49, 0x4D, 0x50, 0x20, 0x46, 0x49, 0x4C,
                    0x45, 0x27, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
