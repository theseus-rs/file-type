use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849269: FileFormat = FileFormat {
    id: 105_849_269,
    puid: "wikidata/105849269",
    name: "YANG file format",
    extensions: &["yang"],
    media_types: &["application/yang"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
