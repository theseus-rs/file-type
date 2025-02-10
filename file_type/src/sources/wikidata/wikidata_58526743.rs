use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58526743: FileFormat = FileFormat {
    id: 58_526_743,
    source_type: SourceType::Wikidata,
    name: "Enigma Transportable File",
    extensions: &["etf", "mus"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x4E, 0x49, 0x47, 0x4D, 0x41, 0x20, 0x54, 0x52, 0x41, 0x4E, 0x53, 0x50,
                    0x4F, 0x52, 0x54, 0x41, 0x42, 0x4C, 0x45, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x0D,
                    0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
