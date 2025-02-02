use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858663: FileFormat = FileFormat {
    id: 105_858_663,
    source_type: SourceType::Wikidata,
    name: "Saved AIM Buddy List",
    extensions: &["blt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x20, 0x7B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
