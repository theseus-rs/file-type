use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855126: FileFormat = FileFormat {
    id: 105_855_126,
    puid: "wikidata/105855126",
    name: "Oracle Binary Form",
    extensions: &["fmb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x4F, 0x53, 0x2E, 0x36, 0x30, 0x30, 0x35, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
