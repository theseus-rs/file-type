use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861093: FileFormat = FileFormat {
    id: 105_861_093,
    source_type: SourceType::Wikidata,
    name: "Wise Installer log",
    extensions: &["log"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x2A, 0x2A, 0x20, 0x20, 0x49, 0x6E, 0x73, 0x74, 0x61, 0x6C, 0x6C, 0x61,
                    0x74, 0x69, 0x6F, 0x6E, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
