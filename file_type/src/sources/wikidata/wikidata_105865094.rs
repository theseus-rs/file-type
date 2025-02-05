use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865094: FileFormat = FileFormat {
    id: 105_865_094,
    source_type: SourceType::Wikidata,
    name: "Black and White 2 paths data",
    extensions: &["pat"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x69, 0x4F, 0x6E, 0x48, 0x65, 0x41, 0x64, 0x50, 0x61, 0x74, 0x68, 0x56,
                    0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
