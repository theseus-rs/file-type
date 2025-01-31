use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853594: FileFormat = FileFormat {
    id: 105_853_594,
    puid: "wikidata/105853594",
    name: "PDS Zip",
    extensions: &["zpk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x44, 0x53, 0x20, 0x5A, 0x49, 0x50, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
