use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850342: FileFormat = FileFormat {
    id: 105_850_342,
    source_type: SourceType::Wikidata,
    name: "X.509v3 security certificate",
    extensions: &["crt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
