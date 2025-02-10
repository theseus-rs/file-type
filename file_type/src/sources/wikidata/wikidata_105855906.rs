use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855906: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_906,
        source_type: SourceType::Wikidata,
        name: "RightWriter configuration (v3-4 DOS)",
        extensions: &["dct"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x69, 0x67, 0x68, 0x74, 0x57, 0x72, 0x69, 0x74, 0x65, 0x72, 0x20,
                        0x28, 0x52, 0x29, 0x20, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75, 0x72,
                        0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
