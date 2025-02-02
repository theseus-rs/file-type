use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856532: FileFormat = FileFormat {
    id: 105_856_532,
    source_type: SourceType::Wikidata,
    name: "VISI-serie CAD/CAM work file",
    extensions: &["wkf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x49, 0x53, 0x49, 0x2D, 0x43, 0x41, 0x44, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
