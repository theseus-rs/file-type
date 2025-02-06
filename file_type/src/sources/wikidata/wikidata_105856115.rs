use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856115: FileFormat = FileFormat {
    id: 105_856_115,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing eXchange Format (var.2/U)",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x20, 0x30, 0x0A, 0x53, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
