use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858760: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_760,
        source_type: SourceType::Wikidata,
        name: "HP ASII GROB bitmap",
        extensions: &["asc", "grb", "gro"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x52, 0x4F, 0x42, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
