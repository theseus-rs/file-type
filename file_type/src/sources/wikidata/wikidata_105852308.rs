use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852308: FileFormat = FileFormat {
    id: 105_852_308,
    puid: "wikidata/105852308",
    name: "SLZ compressed data",
    extensions: &["slz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4C, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
