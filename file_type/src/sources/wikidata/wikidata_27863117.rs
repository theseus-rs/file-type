use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27863117: FileFormat = FileFormat {
    id: 27_863_117,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing, version 2.2",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    signatures: &[Signature {
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
