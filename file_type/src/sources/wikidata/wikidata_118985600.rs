use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118985600: FileFormat = FileFormat {
    id: 118_985_600,
    puid: "wikidata/118985600",
    name: "Simple Relief Format",
    extensions: &["srf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x52, 0x54, 0x43, 0x41, 0x4D, 0x53, 0x52, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
