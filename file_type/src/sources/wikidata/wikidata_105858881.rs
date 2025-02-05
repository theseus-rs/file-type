use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858881: FileFormat = FileFormat {
    id: 105_858_881,
    source_type: SourceType::Wikidata,
    name: "Silicon Graphics 24bit compressed bitmap",
    extensions: &["sgi"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0xDA, 0x00, 0x03, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
