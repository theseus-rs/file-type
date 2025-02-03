use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862012: FileFormat = FileFormat {
    id: 105_862_012,
    source_type: SourceType::Wikidata,
    name: "SunVox module",
    extensions: &["sunvox"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x56, 0x4F, 0x58, 0x00, 0x00, 0x00, 0x00, 0x56, 0x45, 0x52, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
