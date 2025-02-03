use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866534: FileFormat = FileFormat {
    id: 105_866_534,
    source_type: SourceType::Wikidata,
    name: "Massive game data",
    extensions: &["pak"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x41, 0x53, 0x53, 0x49, 0x56, 0x45, 0x46, 0x49, 0x4C, 0x45, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
