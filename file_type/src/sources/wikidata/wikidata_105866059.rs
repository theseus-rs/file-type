use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866059: FileFormat = FileFormat {
    id: 105_866_059,
    puid: "wikidata/105866059",
    name: "PCB ASCII Printed Circuit Board",
    extensions: &["pcb"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x42, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
