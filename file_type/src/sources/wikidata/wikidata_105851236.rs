use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851236: FileFormat = FileFormat {
    id: 105_851_236,
    puid: "wikidata/105851236",
    name: "eMule Web Interface template",
    extensions: &["tmpl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x2D, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
