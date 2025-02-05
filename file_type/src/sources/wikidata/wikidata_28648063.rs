use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28648063: FileFormat = FileFormat {
    id: 28_648_063,
    source_type: SourceType::Wikidata,
    name: "Ein-Ausgabesystem 3",
    extensions: &["eas"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x41, 0x53, 0x33, 0x5F, 0x49, 0x38, 0x52, 0x38,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
