use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863117: FileFormat = FileFormat {
    id: 27_863_117,
    puid: "wikidata/27863117",
    name: "AutoCAD Drawing, version 2.2",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAC, 0x10, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
