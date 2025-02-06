use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866764: FileFormat = FileFormat {
    id: 105_866_764,
    source_type: SourceType::Wikidata,
    name: "Opentech Digital STB main software",
    extensions: &["pgm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x44, 0x53, 0x50, 0x47, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
