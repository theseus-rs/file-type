use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857694: FileFormat = FileFormat {
    id: 105_857_694,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine Dialogue (v1.0)",
    extensions: &["dlg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x4C, 0x47, 0x20, 0x56, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
