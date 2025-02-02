use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852900: FileFormat = FileFormat {
    id: 105_852_900,
    source_type: SourceType::Wikidata,
    name: "Mount and Blade save game",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x52, 0x44, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
