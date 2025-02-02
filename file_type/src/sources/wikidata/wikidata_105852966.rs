use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852966: FileFormat = FileFormat {
    id: 105_852_966,
    source_type: SourceType::Wikidata,
    name: "SNNS network definition",
    extensions: &["net"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4E, 0x4E, 0x53, 0x20, 0x6E, 0x65, 0x74, 0x77, 0x6F, 0x72, 0x6B, 0x20,
                    0x64, 0x65, 0x66, 0x69, 0x6E, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69,
                    0x6C, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
