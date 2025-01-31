use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859064: FileFormat = FileFormat {
    id: 105_859_064,
    puid: "wikidata/105859064",
    name: "NASA PDS labeled bitmap",
    extensions: &["ibg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x4D, 0x41, 0x47, 0x45, 0x49, 0x44, 0x45, 0x4E, 0x54, 0x49, 0x46, 0x49,
                    0x45, 0x52, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
