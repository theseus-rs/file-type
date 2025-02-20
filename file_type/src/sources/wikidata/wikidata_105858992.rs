use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858992: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_992,
        source_type: SourceType::Wikidata,
        name: "Panda3D Bam container",
        extensions: &["bam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x62, 0x6A, 0x00, 0x0A, 0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
