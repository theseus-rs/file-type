use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860058: FileFormat = FileFormat {
    id: 105_860_058,
    source_type: SourceType::Wikidata,
    name: "ARMovie video",
    extensions: &["rpl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x52, 0x4D, 0x6F, 0x76, 0x69, 0x65, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
