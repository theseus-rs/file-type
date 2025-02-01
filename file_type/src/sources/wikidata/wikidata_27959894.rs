use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959894: FileFormat = FileFormat {
    id: 27_959_894,
    puid: "wikidata/27959894",
    name: "Cubase project",
    extensions: &["cpr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x4E, 0x55, 0x4E, 0x44]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
