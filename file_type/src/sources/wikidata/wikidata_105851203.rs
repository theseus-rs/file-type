use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851203: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_203,
        source_type: SourceType::Wikidata,
        name: "TaxACT Ecrypted data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x61, 0x78, 0x41, 0x43, 0x54, 0x20, 0x45, 0x6E, 0x63, 0x72, 0x79,
                        0x70, 0x74, 0x65, 0x64, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
