use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851430: FileFormat = FileFormat {
    id: 105_851_430,
    puid: "wikidata/105851430",
    name: "CodeWarrior Target Data (Big Endian)",
    extensions: &["tdt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x63, 0x6F, 0x6F, 0x6C])],
            },
        }],
    }],
    related_formats: &[],
};
