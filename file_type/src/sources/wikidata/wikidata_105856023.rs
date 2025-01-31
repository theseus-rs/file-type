use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856023: FileFormat = FileFormat {
    id: 105_856_023,
    puid: "wikidata/105856023",
    name: "Dune Firmware File",
    extensions: &["dff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x55, 0x4E, 0x45, 0x20, 0x46, 0x49, 0x52, 0x4D, 0x57, 0x41, 0x52, 0x45,
                    0x20, 0x46, 0x49, 0x4C, 0x45, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
