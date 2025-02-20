use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865148: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_148,
        source_type: SourceType::Wikidata,
        name: "ASIC Project/Configuration",
        extensions: &["cfg", "prj"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x34, 0x0D, 0x0A, 0x4F, 0x42, 0x4A, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
