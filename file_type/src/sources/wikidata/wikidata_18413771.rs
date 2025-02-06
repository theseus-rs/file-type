use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_18413771: FileFormat = FileFormat {
    id: 18_413_771,
    source_type: SourceType::Wikidata,
    name: "Web Open Font Format, version 2",
    extensions: &["woff2"],
    media_types: &["font/woff2"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x4F, 0x46, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
