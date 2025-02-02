use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857563: FileFormat = FileFormat {
    id: 105_857_563,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine Item (v1)",
    extensions: &["itm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x54, 0x4D, 0x20, 0x56, 0x31, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
