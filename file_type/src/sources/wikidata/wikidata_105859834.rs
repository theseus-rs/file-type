use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859834: FileFormat = FileFormat {
    id: 105_859_834,
    source_type: SourceType::Wikidata,
    name: "LABView Virtual Instrument",
    extensions: &["vi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x53, 0x52, 0x43, 0x0D, 0x0A, 0x00, 0x03, 0x4C, 0x56, 0x49, 0x4E, 0x4C,
                    0x42, 0x56, 0x57, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
