use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855520: FileFormat = FileFormat {
    id: 105_855_520,
    source_type: SourceType::Wikidata,
    name: "Freeze compressed data (v2.x)",
    extensions: &["f"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x9F])],
            },
        }],
    }],
    related_formats: &[],
};
