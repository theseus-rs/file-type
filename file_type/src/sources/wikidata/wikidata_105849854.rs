use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849854: FileFormat = FileFormat {
    id: 105_849_854,
    puid: "wikidata/105849854",
    name: "Lotus 123 configuration (V3)",
    extensions: &["cnf"],
    media_types: &["application/vnd.lotus-1-2-3"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x02, 0x14])],
            },
        }],
    }],
    related_formats: &[],
};
