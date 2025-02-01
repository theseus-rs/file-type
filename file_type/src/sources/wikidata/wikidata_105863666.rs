use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863666: FileFormat = FileFormat {
    id: 105_863_666,
    puid: "wikidata/105863666",
    name: "Project: Space Station saved Mission",
    extensions: &["msn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x80, 0x50, 0x80, 0x4D, 0x80, 0x4B, 0x80, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
