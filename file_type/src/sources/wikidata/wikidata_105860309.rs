use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860309: FileFormat = FileFormat {
    id: 105_860_309,
    source_type: SourceType::Wikidata,
    name: "Rave Reports Project",
    extensions: &["rav"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x41, 0x56, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
