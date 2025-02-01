use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856038: FileFormat = FileFormat {
    id: 105_856_038,
    puid: "wikidata/105856038",
    name: "AutoCAD Drawing eXchange Format (var.0/U)",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x30, 0x0A, 0x53, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
