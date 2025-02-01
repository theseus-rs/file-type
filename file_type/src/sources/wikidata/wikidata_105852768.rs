use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852768: FileFormat = FileFormat {
    id: 105_852_768,
    puid: "wikidata/105852768",
    name: "SevenUp bitmap",
    extensions: &["sev"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x65, 0x76, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
