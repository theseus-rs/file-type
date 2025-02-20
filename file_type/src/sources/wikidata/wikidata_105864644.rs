use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864644: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_644,
        source_type: SourceType::Wikidata,
        name: "NeoPaint Palette",
        extensions: &["pat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x65, 0x6F, 0x50, 0x61, 0x69, 0x6E, 0x74, 0x20, 0x50, 0x61, 0x74,
                        0x74, 0x65, 0x72, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
