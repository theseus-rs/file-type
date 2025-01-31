use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757949: FileFormat = FileFormat {
    id: 28_757_949,
    puid: "wikidata/28757949",
    name: "HA",
    extensions: &["ha"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
