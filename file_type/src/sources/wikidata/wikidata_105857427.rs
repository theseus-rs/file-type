use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857427: FileFormat = FileFormat {
    id: 105_857_427,
    puid: "wikidata/105857427",
    name: "Acrobat Distiller Job Options",
    extensions: &["joboptions"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
