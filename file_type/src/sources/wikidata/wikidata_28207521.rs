use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207521: FileFormat = FileFormat {
    id: 28_207_521,
    source_type: SourceType::Wikidata,
    name: "WPB",
    extensions: &["wpb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x50, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
