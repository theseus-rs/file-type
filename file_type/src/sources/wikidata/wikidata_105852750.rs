use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852750: FileFormat = FileFormat {
    id: 105_852_750,
    puid: "wikidata/105852750",
    name: "Panorama Settings",
    extensions: &["set"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x72, 0x61, 0x77, 0x4D, 0x6F, 0x64, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
