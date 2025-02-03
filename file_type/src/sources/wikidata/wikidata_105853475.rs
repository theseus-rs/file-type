use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853475: FileFormat = FileFormat {
    id: 105_853_475,
    source_type: SourceType::Wikidata,
    name: "Zortrax Z-Code",
    extensions: &["zcode"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x43, 0x6F, 0x64, 0x65, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
