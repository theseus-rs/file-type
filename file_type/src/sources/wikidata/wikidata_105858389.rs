use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858389: FileFormat = FileFormat {
    id: 105_858_389,
    source_type: SourceType::Wikidata,
    name: "Envoy document (v1)",
    extensions: &["evy"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x32, 0x5E, 0x10, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
