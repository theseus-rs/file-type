use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853143: FileFormat = FileFormat {
    id: 105_853_143,
    source_type: SourceType::Wikidata,
    name: "K-Spreadsheet (v3/4)",
    extensions: &["spd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4C, 0x49, 0x46, 0x46, 0x20, 0x48, 0x61, 0x72, 0x6B, 0x65, 0x72, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
