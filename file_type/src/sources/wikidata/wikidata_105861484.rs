use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861484: FileFormat = FileFormat {
    id: 105_861_484,
    source_type: SourceType::Wikidata,
    name: "MathWorks License",
    extensions: &["lic"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D,
                    0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E,
                    0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x2D,
                    0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
