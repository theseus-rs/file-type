use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866382: FileFormat = FileFormat {
    id: 105_866_382,
    puid: "wikidata/105866382",
    name: "GoBe Productive Document (gen)",
    extensions: &["pve"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4F, 0x42, 0x45, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
