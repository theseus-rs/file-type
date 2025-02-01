use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72960914: FileFormat = FileFormat {
    id: 72_960_914,
    puid: "wikidata/72960914",
    name: "Pure Data patch",
    extensions: &["pd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x4E, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
