use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860998: FileFormat = FileFormat {
    id: 105_860_998,
    puid: "wikidata/105860998",
    name: "Lecturnity Player file",
    extensions: &["lpd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x50, 0x46, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
