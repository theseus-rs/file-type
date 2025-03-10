use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856658: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_658,
        source_type: SourceType::Wikidata,
        name: "WDDX packet",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x77, 0x64, 0x64, 0x78, 0x50, 0x61, 0x63, 0x6B, 0x65, 0x74, 0x20,
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
