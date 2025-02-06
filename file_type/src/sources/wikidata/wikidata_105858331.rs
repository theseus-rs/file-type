use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858331: FileFormat = FileFormat {
    id: 105_858_331,
    source_type: SourceType::Wikidata,
    name: "Elastic Reality project data",
    extensions: &["er"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x45, 0x6C, 0x61, 0x73, 0x74, 0x69, 0x63, 0x20, 0x52, 0x65, 0x61, 0x6C,
                    0x69, 0x74, 0x79, 0x20, 0x45, 0x52, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46,
                    0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
