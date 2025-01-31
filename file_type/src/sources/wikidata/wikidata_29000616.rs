use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000616: FileFormat = FileFormat {
    id: 29_000_616,
    puid: "wikidata/29000616",
    name: "Google Chrome Extension",
    extensions: &["crx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x72, 0x32, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
