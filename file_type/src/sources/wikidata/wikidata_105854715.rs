use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854715: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_715,
        source_type: SourceType::Wikidata,
        name: "PAKLEO compressed archive",
        extensions: &["pll"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x45, 0x4F, 0x4C, 0x5A, 0x57, 0x20, 0x2D, 0x20, 0x28, 0x63, 0x29,
                        0x20, 0x4C, 0x65, 0x6F, 0x6E, 0x61, 0x72, 0x64, 0x75, 0x73, 0x20, 0x4C,
                        0x65, 0x6F, 0x6E, 0x61, 0x72, 0x64, 0x69, 0x20, 0x31, 0x39, 0x39, 0x33,
                        0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
