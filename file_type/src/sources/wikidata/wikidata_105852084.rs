use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852084: FileFormat = FileFormat {
    id: 105_852_084,
    source_type: SourceType::Wikidata,
    name: "Multibit Bitcoin blockchain",
    extensions: &["spvchain"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x56, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
