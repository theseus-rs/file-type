use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28770288: FileFormat = FileFormat {
    id: 28_770_288,
    source_type: SourceType::Wikidata,
    name: "LBR",
    extensions: &["lbr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x57, 0x42, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
