use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866504: FileFormat = FileFormat {
    id: 105_866_504,
    puid: "wikidata/105866504",
    name: "Print Magic Graphic",
    extensions: &["pmg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x47, 0x52, 0x41, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
