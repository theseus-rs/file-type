use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859295: FileFormat = FileFormat {
    id: 105_859_295,
    puid: "wikidata/105859295",
    name: "Playstation 3 icon",
    extensions: &["gim"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x49, 0x47, 0x2E, 0x30, 0x30, 0x2E, 0x31, 0x50, 0x53, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
