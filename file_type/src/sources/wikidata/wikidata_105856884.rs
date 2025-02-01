use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856884: FileFormat = FileFormat {
    id: 105_856_884,
    puid: "wikidata/105856884",
    name: "Gerber format (with rem)",
    extensions: &["gbr"],
    media_types: &["application/vnd.gerber"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x30, 0x34, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
