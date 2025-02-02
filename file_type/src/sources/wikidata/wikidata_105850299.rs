use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850299: FileFormat = FileFormat {
    id: 105_850_299,
    source_type: SourceType::Wikidata,
    name: "IntroCAD drawing",
    extensions: &["cad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x12, 0xD6, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
