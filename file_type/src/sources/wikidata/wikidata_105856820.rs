use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856820: FileFormat = FileFormat {
    id: 105_856_820,
    puid: "wikidata/105856820",
    name: "Guitar Pro 6 tablature (non compressed)",
    extensions: &["gpx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x43, 0x46, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
