use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857566: FileFormat = FileFormat {
    id: 105_857_566,
    source_type: SourceType::Wikidata,
    name: "Alpha Four Index Definition",
    extensions: &["idn"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x34, 0x06, 0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
