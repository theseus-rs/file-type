use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47538013: FileFormat = FileFormat {
    id: 47_538_013,
    puid: "wikidata/47538013",
    name: "CDX Internet Archive Index format",
    extensions: &["cdx"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x44, 0x58, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
