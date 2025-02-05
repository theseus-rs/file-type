use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857911: FileFormat = FileFormat {
    id: 105_857_911,
    source_type: SourceType::Wikidata,
    name: "Tektronix instrument data file",
    extensions: &["isf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3A, 0x57, 0x46, 0x4D, 0x50, 0x52, 0x45, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
