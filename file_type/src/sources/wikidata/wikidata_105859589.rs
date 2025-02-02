use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859589: FileFormat = FileFormat {
    id: 105_859_589,
    source_type: SourceType::Wikidata,
    name: "trsvid TV3 video",
    extensions: &["tv3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x57, 0x50, 0xD6, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
