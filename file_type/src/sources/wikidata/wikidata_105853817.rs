use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853817: FileFormat = FileFormat {
    id: 105_853_817,
    source_type: SourceType::Wikidata,
    name: "UNIX Compressed data",
    extensions: &["z"],
    media_types: &["application/x-compress"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x9D, 0x90])],
            },
        }],
    }],
    related_formats: &[],
};
