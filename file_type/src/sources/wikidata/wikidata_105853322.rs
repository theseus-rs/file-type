use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853322: FileFormat = FileFormat {
    id: 105_853_322,
    source_type: SourceType::Wikidata,
    name: "Sonnet Graph",
    extensions: &["sgr"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x54, 0x59, 0x50, 0x20, 0x53, 0x4F, 0x4E, 0x47, 0x52, 0x41, 0x50, 0x48,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
