use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857581: FileFormat = FileFormat {
    id: 105_857_581,
    source_type: SourceType::Wikidata,
    name: "Ivona voice info",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x53, 0x45, 0x46, 0x00, 0x00, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
