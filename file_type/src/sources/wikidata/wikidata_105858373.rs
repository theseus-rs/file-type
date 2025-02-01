use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858373: FileFormat = FileFormat {
    id: 105_858_373,
    puid: "wikidata/105858373",
    name: "Encarta Encyclopedia Yearbook and Web Links update",
    extensions: &["eyb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x59, 0x42, 0x24])],
            },
        }],
    }],
    related_formats: &[],
};
