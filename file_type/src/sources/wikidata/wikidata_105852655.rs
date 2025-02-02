use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852655: FileFormat = FileFormat {
    id: 105_852_655,
    source_type: SourceType::Wikidata,
    name: "PS2 Icon",
    extensions: &["sys"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x53, 0x32, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
