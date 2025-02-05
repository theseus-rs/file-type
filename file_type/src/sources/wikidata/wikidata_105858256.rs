use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858256: FileFormat = FileFormat {
    id: 105_858_256,
    source_type: SourceType::Wikidata,
    name: "LG EPK v1 firmware package",
    extensions: &["epk"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x65, 0x70, 0x61, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
