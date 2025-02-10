use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867008: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_008,
        source_type: SourceType::Wikidata,
        name: "KiCad EESchema Netlist",
        extensions: &["net"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x65, 0x78, 0x70, 0x6F, 0x72, 0x74, 0x20, 0x28, 0x76, 0x65, 0x72,
                        0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
