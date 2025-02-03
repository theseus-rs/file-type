use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857869: FileFormat = FileFormat {
    id: 105_857_869,
    source_type: SourceType::Wikidata,
    name: "ICC Animation",
    extensions: &["icc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x69, 0x63, 0x63, 0x20, 0x61, 0x6E, 0x69, 0x6D, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
