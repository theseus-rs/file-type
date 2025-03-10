use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855699: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_699,
        source_type: SourceType::Wikidata,
        name: "ObjectStore backup or archive image",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x62, 0x4A, 0x65, 0x43, 0x74, 0x53, 0x74, 0x4F, 0x72, 0x45, 0x64,
                        0x41, 0x74, 0x41, 0x62, 0x41, 0x73, 0x45, 0x62, 0x41, 0x63, 0x4B, 0x75,
                        0x50, 0x69, 0x4D, 0x61, 0x47, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
