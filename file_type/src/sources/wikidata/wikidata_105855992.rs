use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855992: FileFormat = FileFormat {
    id: 105_855_992,
    source_type: SourceType::Wikidata,
    name: "DA's Layout project",
    extensions: &["dip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x41, 0x4C, 0x41, 0x59, 0x4F, 0x55, 0x54, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
