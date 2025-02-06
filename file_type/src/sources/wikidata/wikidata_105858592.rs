use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858592: FileFormat = FileFormat {
    id: 105_858_592,
    source_type: SourceType::Wikidata,
    name: "Cambridge Z88 BASIC tokenized source",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
