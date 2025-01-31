use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856309: FileFormat = FileFormat {
    id: 105_856_309,
    puid: "wikidata/105856309",
    name: "AutoCAD R2.22-20xx Drawing (generic)",
    extensions: &["dwg"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x31, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
