use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859938: FileFormat = FileFormat {
    id: 105_859_938,
    puid: "wikidata/105859938",
    name: "Weston CAPture video (BE)",
    extensions: &["wcap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x43, 0x41, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
