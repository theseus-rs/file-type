use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852259: FileFormat = FileFormat {
    id: 105_852_259,
    source_type: SourceType::Wikidata,
    name: "PageRender3D Script",
    extensions: &["script"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
