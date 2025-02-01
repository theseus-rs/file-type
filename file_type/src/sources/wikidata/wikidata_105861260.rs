use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861260: FileFormat = FileFormat {
    id: 105_861_260,
    puid: "wikidata/105861260",
    name: "Lingoes Dictionary",
    extensions: &["ld2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3F, 0x4C, 0x44, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
