use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865529: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_529,
        source_type: SourceType::Wikidata,
        name: "NeoPaint Printer Driver",
        extensions: &["prd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x90, 0xCA, 0x4E, 0x65, 0x6F, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x20, 0x50,
                        0x72, 0x69, 0x6E, 0x74, 0x65, 0x72, 0x20, 0x44, 0x72, 0x69, 0x76, 0x65,
                        0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
