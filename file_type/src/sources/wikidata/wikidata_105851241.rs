use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851241: FileFormat = FileFormat {
    id: 105_851_241,
    puid: "wikidata/105851241",
    name: "Taikojiro Song Map",
    extensions: &["tja"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x54, 0x4C, 0x45, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
