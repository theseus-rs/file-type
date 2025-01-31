use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000585: FileFormat = FileFormat {
    id: 29_000_585,
    puid: "wikidata/29000585",
    name: "Dalvik VM DEX",
    extensions: &["dex"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x65, 0x78, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
