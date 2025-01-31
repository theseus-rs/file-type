use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855742: FileFormat = FileFormat {
    id: 105_855_742,
    puid: "wikidata/105855742",
    name: "GEM-View Dither",
    extensions: &["dit"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x26, 0x57, 0x32, 0x35, 0x36, 0x00, 0x10, 0x00, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
