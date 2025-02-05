use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857427: FileFormat = FileFormat {
    id: 105_857_427,
    source_type: SourceType::Wikidata,
    name: "Acrobat Distiller Job Options",
    extensions: &["joboptions"],
    media_types: &[],
    signatures: &[Signature {
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
