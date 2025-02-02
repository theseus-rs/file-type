use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850584: FileFormat = FileFormat {
    id: 105_850_584,
    source_type: SourceType::Wikidata,
    name: "Copper Colourz! File",
    extensions: &["ccf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x43, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
