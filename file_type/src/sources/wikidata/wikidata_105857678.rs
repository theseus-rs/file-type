use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857678: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_678,
        source_type: SourceType::Wikidata,
        name: "Famicom Disk System disk image",
        extensions: &["fds"],
        media_types: &["application/x-fds-disk"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x44, 0x53, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
