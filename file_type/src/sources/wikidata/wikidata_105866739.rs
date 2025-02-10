use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866739: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_739,
        source_type: SourceType::Wikidata,
        name: "Pure Compound Data",
        extensions: &["pcd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x04, 0x2F, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x08, 0x08, 0x08,
                        0x43, 0x68, 0x65, 0x6D, 0x4C, 0x69, 0x62, 0x20, 0x50, 0x75, 0x72, 0x65,
                        0x20, 0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x6E, 0x65, 0x6E, 0x74, 0x20, 0x44,
                        0x61, 0x74, 0x61, 0x20, 0x4C, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x2E,
                        0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
