use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856200: FileFormat = FileFormat {
    id: 105_856_200,
    puid: "wikidata/105856200",
    name: "AutoCAD R2.22 Drawing (new)",
    extensions: &["dwg"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x31, 0x30, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
