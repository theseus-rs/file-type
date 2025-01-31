use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859503: FileFormat = FileFormat {
    id: 105_859_503,
    puid: "wikidata/105859503",
    name: "Dalet Volume info",
    extensions: &["vol"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE5, 0xC5])],
            },
        }],
    }],
    related_formats: &[],
};
