use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7520656: FileFormat = FileFormat {
    id: 7_520_656,
    source_type: SourceType::Wikidata,
    name: "Simple Data Format",
    extensions: &["sdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x44, 0x46, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
