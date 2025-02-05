use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858112: FileFormat = FileFormat {
    id: 105_858_112,
    source_type: SourceType::Wikidata,
    name: "Imagine Effect",
    extensions: &["ifx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x51])],
            },
        }],
    }],
    related_formats: &[],
};
