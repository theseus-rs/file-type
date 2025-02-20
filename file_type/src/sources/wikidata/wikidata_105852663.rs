use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852663: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_663,
        source_type: SourceType::Wikidata,
        name: "KiCad Schematics",
        extensions: &["sch"],
        media_types: &["application/x-kicad-schematic"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x45, 0x53, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x20, 0x53, 0x63, 0x68,
                        0x65, 0x6D, 0x61, 0x74, 0x69, 0x63, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
