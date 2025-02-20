use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852388: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_388,
        source_type: SourceType::Wikidata,
        name: "SPSS for Windows Data",
        extensions: &["sav"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x24, 0x46, 0x4C, 0x32, 0x40, 0x28, 0x23, 0x29, 0x20, 0x53, 0x50, 0x53,
                        0x53, 0x20, 0x44, 0x41, 0x54, 0x41, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
