use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853548: FileFormat = FileFormat {
    id: 105_853_548,
    source_type: SourceType::Wikidata,
    name: "ZenGL Font",
    extensions: &["zfi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x47, 0x4C, 0x5F, 0x46, 0x4F, 0x4E, 0x54, 0x5F, 0x49, 0x4E, 0x46, 0x4F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
