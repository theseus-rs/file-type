use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81192847: FileFormat = FileFormat {
    id: 81_192_847,
    puid: "wikidata/81192847",
    name: "The Bee Archiver compressed archive",
    extensions: &["bee"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x65, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
