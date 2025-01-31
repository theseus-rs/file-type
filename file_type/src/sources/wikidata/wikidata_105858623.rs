use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858623: FileFormat = FileFormat {
    id: 105_858_623,
    puid: "wikidata/105858623",
    name: "Hard Color Map bitmap",
    extensions: &["hcm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x43, 0x4D, 0x41, 0x38, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
