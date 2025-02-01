use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852715: FileFormat = FileFormat {
    id: 105_852_715,
    puid: "wikidata/105852715",
    name: "Adobe Substance SBSASM",
    extensions: &["sbsasm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x41, 0x4D, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
