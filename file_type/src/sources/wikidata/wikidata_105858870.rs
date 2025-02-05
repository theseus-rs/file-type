use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858870: FileFormat = FileFormat {
    id: 105_858_870,
    source_type: SourceType::Wikidata,
    name: "BAM Index",
    extensions: &["bai"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x41, 0x49, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
