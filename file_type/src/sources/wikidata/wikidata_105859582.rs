use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859582: FileFormat = FileFormat {
    id: 105_859_582,
    source_type: SourceType::Wikidata,
    name: "Visual Studio wizard",
    extensions: &["vsz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x53, 0x57, 0x49, 0x5A, 0x41, 0x52, 0x44, 0x20, 0x37, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
