use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856270: FileFormat = FileFormat {
    id: 105_856_270,
    puid: "wikidata/105856270",
    name: "Digital Sound Studio module",
    extensions: &["dss"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x55, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
