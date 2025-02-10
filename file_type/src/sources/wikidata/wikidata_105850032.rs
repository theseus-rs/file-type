use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850032: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_032,
        source_type: SourceType::Wikidata,
        name: "Celestia 3D model (ASCII)",
        extensions: &["cmod"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x63, 0x65, 0x6C, 0x6D, 0x6F, 0x64, 0x65, 0x6C, 0x5F, 0x5F, 0x61,
                        0x73, 0x63, 0x69, 0x69,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
