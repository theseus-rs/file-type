use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856260: FileFormat = FileFormat {
    id: 105_856_260,
    puid: "wikidata/105856260",
    name: "AutoCAD R2.21 Drawing",
    extensions: &["dwg"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x32, 0x2E, 0x32, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
