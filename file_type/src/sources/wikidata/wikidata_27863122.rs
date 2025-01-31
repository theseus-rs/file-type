use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863122: FileFormat = FileFormat {
    id: 27_863_122,
    puid: "wikidata/27863122",
    name: "AutoCAD Drawing, version 9",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAC, 0x10, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
