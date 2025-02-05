use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857051: FileFormat = FileFormat {
    id: 105_857_051,
    source_type: SourceType::Wikidata,
    name: "Guile Object bytecode (little endian)",
    extensions: &["go"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x4F, 0x4F, 0x46, 0x2D, 0x2D, 0x2D, 0x2D, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
