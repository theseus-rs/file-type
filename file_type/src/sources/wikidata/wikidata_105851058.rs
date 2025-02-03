use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851058: FileFormat = FileFormat {
    id: 105_851_058,
    source_type: SourceType::Wikidata,
    name: "TommySoftware CAD/Draw Library (v1)",
    extensions: &["tvl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x6F, 0x6D, 0x6D, 0x79, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65,
                    0x20, 0x54, 0x56, 0x4C, 0x20, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
