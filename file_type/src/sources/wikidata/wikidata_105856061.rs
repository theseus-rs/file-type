use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856061: FileFormat = FileFormat {
    id: 105_856_061,
    puid: "wikidata/105856061",
    name: "AutoCAD R2.22 Drawing (old)",
    extensions: &["dwg"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x32, 0x2E, 0x32, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
