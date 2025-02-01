use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849864: FileFormat = FileFormat {
    id: 105_849_864,
    puid: "wikidata/105849864",
    name: "Source Insight Custom Language File",
    extensions: &["clf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x11, 0x09, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
