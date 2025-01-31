use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866764: FileFormat = FileFormat {
    id: 105_866_764,
    puid: "wikidata/105866764",
    name: "Opentech Digital STB main software",
    extensions: &["pgm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
