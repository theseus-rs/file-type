use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862500: FileFormat = FileFormat {
    id: 105_862_500,
    source_type: SourceType::Wikidata,
    name: "Music Publisher Score",
    extensions: &["mup"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x23])],
            },
        }],
    }],
    related_formats: &[],
};
