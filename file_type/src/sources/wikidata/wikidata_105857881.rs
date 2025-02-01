use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857881: FileFormat = FileFormat {
    id: 105_857_881,
    puid: "wikidata/105857881",
    name: "MySQL Dictionary File (ISAM)",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x01, 0x07, 0x07])],
            },
        }],
    }],
    related_formats: &[],
};
