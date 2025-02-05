use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857441: FileFormat = FileFormat {
    id: 105_857_441,
    source_type: SourceType::Wikidata,
    name: "2BIT DNA sequences (LE)",
    extensions: &["2bit"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x27, 0x41, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
