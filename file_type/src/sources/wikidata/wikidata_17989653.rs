use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17989653: FileFormat = FileFormat {
    id: 17_989_653,
    source_type: SourceType::Wikidata,
    name: "BOM",
    extensions: &["bom"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x4F, 0x4D, 0x53, 0x74, 0x6F, 0x72, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
