use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857639: FileFormat = FileFormat {
    id: 105_857_639,
    puid: "wikidata/105857639",
    name: "CopyTape intermediate data format",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x50, 0x54, 0x50, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
