use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71832451: FileFormat = FileFormat {
    id: 71_832_451,
    puid: "wikidata/71832451",
    name: "CorelDraw Drawing, version 1",
    extensions: &["cdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x4C, 0x65, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
