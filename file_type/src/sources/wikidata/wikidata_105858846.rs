use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858846: FileFormat = FileFormat {
    id: 105_858_846,
    source_type: SourceType::Wikidata,
    name: "Corel Photo Paint bitmap (v7)",
    extensions: &["cpt"],
    media_types: &["image/x-corel-cpt"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x50, 0x54, 0x37, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
