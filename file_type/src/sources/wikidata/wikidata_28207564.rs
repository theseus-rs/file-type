use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207564: FileFormat = FileFormat {
    id: 28_207_564,
    source_type: SourceType::Wikidata,
    name: "Bennet Yee's face format",
    extensions: &["bm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
