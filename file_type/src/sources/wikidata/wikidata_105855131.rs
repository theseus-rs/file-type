use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855131: FileFormat = FileFormat {
    id: 105_855_131,
    puid: "wikidata/105855131",
    name: "Flow Cytometry Standard format",
    extensions: &["fcs"],
    media_types: &["application/vnd.isac.fcs"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x43, 0x53, 0x32, 0x2E, 0x30, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
