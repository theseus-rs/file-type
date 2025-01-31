use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866325: FileFormat = FileFormat {
    id: 105_866_325,
    puid: "wikidata/105866325",
    name: "smARTWORK Printed Circuit Board project",
    extensions: &["pcb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x77, 0x62, 0x63, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
