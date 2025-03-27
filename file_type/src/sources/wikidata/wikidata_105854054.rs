use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854054: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_054,
        source_type: SourceType::Wikidata,
        name: "Amiga Disk image File (generic)",
        extensions: &["adf"],
        media_types: &["application/x-amiga-disk-format"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x4F, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
