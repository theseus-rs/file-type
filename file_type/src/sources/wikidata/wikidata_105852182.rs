use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852182: FileFormat = FileFormat {
    id: 105_852_182,
    source_type: SourceType::Wikidata,
    name: "SuperTux Sprite",
    extensions: &["sprite"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x28, 0x73])],
            },
        }],
    }],
    related_formats: &[],
};
