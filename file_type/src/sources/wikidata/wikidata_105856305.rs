use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856305: FileFormat = FileFormat {
    id: 105_856_305,
    source_type: SourceType::Wikidata,
    name: "MacDraw drawing",
    extensions: &["drw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x44, 0x6F, 0x63, 0x44, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
