use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852495: FileFormat = FileFormat {
    id: 105_852_495,
    source_type: SourceType::Wikidata,
    name: "SEG-2 data",
    extensions: &["seg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
