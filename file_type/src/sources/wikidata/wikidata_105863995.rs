use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863995: FileFormat = FileFormat {
    id: 105_863_995,
    puid: "wikidata/105863995",
    name: "Cakewalk Macro (DOS)",
    extensions: &["mac"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x31, 0x32, 0x54, 0x4F, 0x4E, 0x45, 0x2D, 0x4D, 0x41, 0x43, 0x52, 0x4F, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
