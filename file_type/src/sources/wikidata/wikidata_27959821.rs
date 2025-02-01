use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959821: FileFormat = FileFormat {
    id: 27_959_821,
    puid: "wikidata/27959821",
    name: "Ableton Warp Analysis",
    extensions: &["asd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x06, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
