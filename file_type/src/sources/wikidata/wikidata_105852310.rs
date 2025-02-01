use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852310: FileFormat = FileFormat {
    id: 105_852_310,
    puid: "wikidata/105852310",
    name: "BlastEm",
    extensions: &["state"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x4C, 0x53, 0x54, 0x53, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
