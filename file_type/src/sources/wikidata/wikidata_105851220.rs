use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851220: FileFormat = FileFormat {
    id: 105_851_220,
    source_type: SourceType::Wikidata,
    name: "TuneUp Styler Icon pack",
    extensions: &["tip"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
