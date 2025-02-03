use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117222357: FileFormat = FileFormat {
    id: 117_222_357,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 12",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAC, 0x10, 0x12])],
            },
        }],
    }],
    related_formats: &[],
};
