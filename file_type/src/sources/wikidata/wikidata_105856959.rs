use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856959: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_959,
        source_type: SourceType::Wikidata,
        name: "Golden Software Boundary data",
        extensions: &["gsb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x6F, 0x6C, 0x64, 0x65, 0x6E, 0x20, 0x53, 0x6F, 0x66, 0x74, 0x77,
                        0x61, 0x72, 0x65, 0x20, 0x42, 0x6F, 0x75, 0x6E, 0x64, 0x61, 0x72, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
