use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857126: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_126,
        source_type: SourceType::Wikidata,
        name: "Build Engine GRP container",
        extensions: &["grp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4B, 0x65, 0x6E, 0x53, 0x69, 0x6C, 0x76, 0x65, 0x72, 0x6D, 0x61, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
