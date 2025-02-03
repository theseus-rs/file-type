use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856102: FileFormat = FileFormat {
    id: 105_856_102,
    source_type: SourceType::Wikidata,
    name: "AutoCAD R1.0 Drawing",
    extensions: &["dwg"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x43, 0x30, 0x2E, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
