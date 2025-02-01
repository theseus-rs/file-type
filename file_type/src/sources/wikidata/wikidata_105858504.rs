use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858504: FileFormat = FileFormat {
    id: 105_858_504,
    puid: "wikidata/105858504",
    name: "Encrypted Multi-Picture Object bitmap",
    extensions: &["empo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x4D, 0x50, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
