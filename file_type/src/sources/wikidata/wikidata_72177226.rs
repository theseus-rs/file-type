use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72177226: FileType = FileType {
    file_format: &FileFormat {
        id: 72_177_226,
        source_type: SourceType::Wikidata,
        name: "KiCad PCB",
        extensions: &["KICAD_PCB"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x6B, 0x69, 0x63, 0x61, 0x64, 0x5F, 0x70, 0x63, 0x62, 0x20, 0x28,
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
