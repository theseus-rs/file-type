use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854567: FileFormat = FileFormat {
    id: 105_854_567,
    source_type: SourceType::Wikidata,
    name: "freeCAD assembly",
    extensions: &["asm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4F, 0x53, 0x53, 0x20, 0x37, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
