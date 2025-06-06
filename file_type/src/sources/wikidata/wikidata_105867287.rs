use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_287,
        source_type: SourceType::Wikidata,
        name: "NeoGeo game cartridge (var 2)",
        extensions: &["ngc", "ngp", "npc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4F, 0x50, 0x59, 0x52, 0x49, 0x47, 0x48, 0x54, 0x20, 0x42, 0x59,
                        0x20, 0x53, 0x4E, 0x4B, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x4F, 0x52, 0x41,
                        0x54, 0x49, 0x4F, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
