use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859280: FileFormat = FileFormat {
    id: 105_859_280,
    source_type: SourceType::Wikidata,
    name: "BWTC32Key encoded",
    extensions: &["b3k"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0xE4, 0xB4, 0x80])],
            },
        }],
    }],
    related_formats: &[],
};
