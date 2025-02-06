use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855892: FileFormat = FileFormat {
    id: 105_855_892,
    source_type: SourceType::Wikidata,
    name: "DER encoded X509 Certificate",
    extensions: &["der"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x30, 0x82])],
            },
        }],
    }],
    related_formats: &[],
};
