use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852391: FileFormat = FileFormat {
    id: 105_852_391,
    source_type: SourceType::Wikidata,
    name: "Motorola phone skin info",
    extensions: &["ski"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4B, 0x49, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
