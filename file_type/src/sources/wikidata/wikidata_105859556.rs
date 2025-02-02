use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859556: FileFormat = FileFormat {
    id: 105_859_556,
    source_type: SourceType::Wikidata,
    name: "Nintendo DS MPEG Video",
    extensions: &["dpg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x50, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
