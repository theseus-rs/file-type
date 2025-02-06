use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854551: FileFormat = FileFormat {
    id: 105_854_551,
    source_type: SourceType::Wikidata,
    name: "AutoREALM Map",
    extensions: &["aur"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x75, 0x74, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
