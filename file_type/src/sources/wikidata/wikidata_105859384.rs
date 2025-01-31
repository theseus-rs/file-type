use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859384: FileFormat = FileFormat {
    id: 105_859_384,
    puid: "wikidata/105859384",
    name: "Avira AntiVir quarantined",
    extensions: &["qua"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x6E, 0x74, 0x69, 0x56, 0x69, 0x72, 0x20, 0x51, 0x75, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
