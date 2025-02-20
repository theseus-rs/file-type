use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123353803: FileType = FileType {
    file_format: &FileFormat {
        id: 123_353_803,
        source_type: SourceType::Wikidata,
        name: "C2PA Manifest",
        extensions: &["c2pa"],
        media_types: &["application/x-c2pa-manifest-store"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6A, 0x75, 0x6D, 0x64, 0x63, 0x32, 0x70, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
