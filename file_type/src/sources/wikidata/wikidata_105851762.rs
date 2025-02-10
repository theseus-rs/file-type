use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851762: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_762,
        source_type: SourceType::Wikidata,
        name: "AmiAtlas Way data",
        extensions: &["spec"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x49, 0x47, 0x41, 0x5F, 0x41, 0x54, 0x4C, 0x41, 0x53, 0x5F,
                        0x57, 0x41, 0x59,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
