use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852947: FileFormat = FileFormat {
    id: 105_852_947,
    puid: "wikidata/105852947",
    name: "Snes9x movie capture",
    extensions: &["smv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4D, 0x56, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
