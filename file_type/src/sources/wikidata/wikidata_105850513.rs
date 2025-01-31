use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850513: FileFormat = FileFormat {
    id: 105_850_513,
    puid: "wikidata/105850513",
    name: "Callus savestate",
    extensions: &["cs0"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x53, 0x54, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
