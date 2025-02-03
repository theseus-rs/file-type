use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853772: FileFormat = FileFormat {
    id: 105_853_772,
    source_type: SourceType::Wikidata,
    name: "Compressed Square Wave (v1.1)",
    extensions: &["csw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x53, 0x71,
                    0x75, 0x61, 0x72, 0x65, 0x20, 0x57, 0x61, 0x76, 0x65, 0x1A, 0x01, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
