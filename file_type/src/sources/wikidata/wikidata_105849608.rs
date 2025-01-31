use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849608: FileFormat = FileFormat {
    id: 105_849_608,
    puid: "wikidata/105849608",
    name: "Dos Navigator 2 settings",
    extensions: &["cfg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x4E, 0x32, 0x20, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
                    0x74, 0x69, 0x6F, 0x6E, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
