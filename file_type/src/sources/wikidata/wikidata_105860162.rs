use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860162: FileFormat = FileFormat {
    id: 105_860_162,
    source_type: SourceType::Wikidata,
    name: "Miasmata game data",
    extensions: &["rs5"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x46, 0x49, 0x4C, 0x45, 0x48, 0x44, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
