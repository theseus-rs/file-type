use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858595: FileFormat = FileFormat {
    id: 105_858_595,
    source_type: SourceType::Wikidata,
    name: "Naive Image format NIA animated bitmaps",
    extensions: &["nia"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0xC3, 0xAF, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
