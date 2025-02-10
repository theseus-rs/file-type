use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_15631639: FileFormat = FileFormat {
    id: 15_631_639,
    source_type: SourceType::Wikidata,
    name: "qgs",
    extensions: &["qgs"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
