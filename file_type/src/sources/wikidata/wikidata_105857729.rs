use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857729: FileFormat = FileFormat {
    id: 105_857_729,
    source_type: SourceType::Wikidata,
    name: "Brother Word Processors disk image (240K)",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x89, 0x85, 0x1E, 0x11])],
            },
        }],
    }],
    related_formats: &[],
};
