use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866525: FileFormat = FileFormat {
    id: 105_866_525,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Deluxe Border",
    extensions: &["pbr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x44, 0x65, 0x6C, 0x75, 0x78, 0x65, 0x2E, 0x50, 0x42, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
