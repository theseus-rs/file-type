use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854143: FileFormat = FileFormat {
    id: 105_854_143,
    puid: "wikidata/105854143",
    name: "Seekable GZIP disk image format",
    extensions: &["sgz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x67, 0x7A])],
            },
        }],
    }],
    related_formats: &[],
};
