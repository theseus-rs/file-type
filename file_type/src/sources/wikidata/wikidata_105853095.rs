use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853095: FileFormat = FileFormat {
    id: 105_853_095,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Shape",
    extensions: &["shx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x2D, 0x38, 0x36, 0x20, 0x73, 0x68,
                    0x61, 0x70, 0x65, 0x73, 0x20, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
