use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849679: FileFormat = FileFormat {
    id: 105_849_679,
    source_type: SourceType::Wikidata,
    name: "Crossword Weaver puzzle",
    extensions: &["cww"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x57, 0x57, 0x46, 0x69, 0x6C, 0x65, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                    0x3D, 0x32, 0x2E, 0x30, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
