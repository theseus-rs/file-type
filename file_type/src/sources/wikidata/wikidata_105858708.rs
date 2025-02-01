use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858708: FileFormat = FileFormat {
    id: 105_858_708,
    puid: "wikidata/105858708",
    name: "Lepton UJG bitmap",
    extensions: &["ujg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x4A])],
            },
        }],
    }],
    related_formats: &[],
};
