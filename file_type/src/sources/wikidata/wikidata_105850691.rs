use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850691: FileFormat = FileFormat {
    id: 105_850_691,
    source_type: SourceType::Wikidata,
    name: "KeyCAD Complete drawing",
    extensions: &["kcf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0F, 0x4B, 0x65, 0x79, 0x43, 0x61, 0x64, 0x20, 0x44, 0x72, 0x61, 0x77, 0x69,
                    0x6E, 0x67, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
