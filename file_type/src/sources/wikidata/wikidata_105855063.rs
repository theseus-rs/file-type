use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855063: FileFormat = FileFormat {
    id: 105_855_063,
    puid: "wikidata/105855063",
    name: "Axon Text File format",
    extensions: &["atf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x54, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
