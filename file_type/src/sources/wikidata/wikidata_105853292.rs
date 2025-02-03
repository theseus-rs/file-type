use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853292: FileFormat = FileFormat {
    id: 105_853_292,
    source_type: SourceType::Wikidata,
    name: "Shift Help info",
    extensions: &["sh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x48, 0x49, 0x46, 0x54, 0x5F, 0x48, 0x45, 0x4C, 0x50, 0x0A, 0x40,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
