use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855210: FileFormat = FileFormat {
    id: 105_855_210,
    source_type: SourceType::Wikidata,
    name: "FLIP Bitmap",
    extensions: &["fbi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF0, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};
