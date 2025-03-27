use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1882110: FileType = FileType {
    file_format: &FileFormat {
        id: 1_882_110,
        source_type: SourceType::Wikidata,
        name: "DOS MZ executable",
        extensions: &["dll", "exe"],
        media_types: &[
            "application/x-dosexec",
            "application/x-ms-dos-executable",
            "application/x-msdos-program",
        ],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
