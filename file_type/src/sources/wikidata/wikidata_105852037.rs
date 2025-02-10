use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852037: FileFormat = FileFormat {
    id: 105_852_037,
    source_type: SourceType::Wikidata,
    name: "Digital Micrograph Script",
    extensions: &["s"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F])],
            },
        }],
    }],
    related_formats: &[],
};
