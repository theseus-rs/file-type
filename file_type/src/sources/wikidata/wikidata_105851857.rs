use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851857: FileFormat = FileFormat {
    id: 105_851_857,
    source_type: SourceType::Wikidata,
    name: "Helix Stronghold Encrypted file",
    extensions: &["saf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x65, 0x6C, 0x69, 0x78, 0x20, 0x53, 0x74, 0x72, 0x6F, 0x6E, 0x67, 0x68,
                    0x6F, 0x6C, 0x64, 0x20, 0x45, 0x6E, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64,
                    0x20, 0x46, 0x69, 0x6C, 0x65, 0x1A, 0x73, 0x61, 0x66, 0x65, 0x25,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
