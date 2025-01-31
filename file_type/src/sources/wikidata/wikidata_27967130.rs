use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967130: FileFormat = FileFormat {
    id: 27_967_130,
    puid: "wikidata/27967130",
    name: "CyberTracker instrument",
    extensions: &["ci"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x04, 0x4E, 0x4E, 0x54, 0x52, 0x4B, 0x49, 0x4E, 0x53, 0x00, 0x01, 0x0A,
                    0x02, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
