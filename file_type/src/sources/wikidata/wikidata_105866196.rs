use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866196: FileFormat = FileFormat {
    id: 105_866_196,
    puid: "wikidata/105866196",
    name: "OS/2 Pointer (color)",
    extensions: &["ptr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x50, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
