use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855831: FileFormat = FileFormat {
    id: 105_855_831,
    puid: "wikidata/105855831",
    name: "AutoCAD R2.10 Drawing",
    extensions: &["dwg"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x32, 0x2E, 0x31, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
