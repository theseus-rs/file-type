use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858998: FileFormat = FileFormat {
    id: 105_858_998,
    source_type: SourceType::Wikidata,
    name: "Zoner BMI Bitmap",
    extensions: &["bmi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x6F, 0x6E, 0x65, 0x72, 0x42, 0x4D, 0x49,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
