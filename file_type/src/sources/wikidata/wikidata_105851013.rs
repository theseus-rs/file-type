use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851013: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_013,
        source_type: SourceType::Wikidata,
        name: "MeshCAM Tool Path Settings",
        extensions: &["tps"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x22, 0x47, 0x4C, 0x4F, 0x42, 0x41, 0x4C, 0x22, 0x3A, 0x7B, 0x22,
                        0x4D, 0x61, 0x63, 0x68, 0x69, 0x6E, 0x65, 0x4D, 0x61, 0x72, 0x67, 0x69,
                        0x6E, 0x22, 0x3A, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
