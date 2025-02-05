use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865291: FileFormat = FileFormat {
    id: 105_865_291,
    source_type: SourceType::Wikidata,
    name: "Lightwork Pattern",
    extensions: &["pattern"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x69, 0x78, 0x65, 0x6C, 0x73, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
