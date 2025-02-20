use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859109: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_109,
        source_type: SourceType::Wikidata,
        name: "AmiAtlas Borders data",
        extensions: &["borders"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x49, 0x47, 0x41, 0x5F, 0x41, 0x54, 0x4C, 0x41, 0x53, 0x5F,
                        0x42, 0x4F, 0x52, 0x44, 0x45, 0x52, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
