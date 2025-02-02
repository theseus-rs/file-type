use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849580: FileFormat = FileFormat {
    id: 105_849_580,
    source_type: SourceType::Wikidata,
    name: "Varsity Scripsit Configuration",
    extensions: &["cnf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x61, 0x72, 0x73, 0x69, 0x74, 0x79, 0x20, 0x63, 0x6F, 0x6E, 0x66, 0x69,
                    0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
