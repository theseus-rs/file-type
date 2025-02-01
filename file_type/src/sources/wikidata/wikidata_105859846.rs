use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859846: FileFormat = FileFormat {
    id: 105_859_846,
    puid: "wikidata/105859846",
    name: "Bethesda Softworks video",
    extensions: &["vid"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x49, 0x44, 0x00, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
