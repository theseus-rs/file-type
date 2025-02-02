use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851652: FileFormat = FileFormat {
    id: 105_851_652,
    source_type: SourceType::Wikidata,
    name: "MoI STereoLithography (binary)",
    extensions: &["stl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x6F, 0x49, 0x20, 0x42, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x20, 0x53, 0x54,
                    0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
