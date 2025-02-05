use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858529: FileFormat = FileFormat {
    id: 105_858_529,
    source_type: SourceType::Wikidata,
    name: "CloudCompare BIN V2 format",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x43, 0x42, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
