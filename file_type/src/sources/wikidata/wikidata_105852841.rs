use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852841: FileFormat = FileFormat {
    id: 105_852_841,
    source_type: SourceType::Wikidata,
    name: "PlayStation Sprite Editor project File",
    extensions: &["sdf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x4D, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
