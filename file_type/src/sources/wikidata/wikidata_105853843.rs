use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853843: FileFormat = FileFormat {
    id: 105_853_843,
    source_type: SourceType::Wikidata,
    name: "Casio Pocket Viewer Add-in Data file",
    extensions: &["adt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x53, 0x49, 0x4F, 0x20, 0x41, 0x44, 0x44, 0x49, 0x4E, 0x20, 0x44,
                    0x41, 0x54, 0x41, 0x46, 0x49, 0x4C, 0x45, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
