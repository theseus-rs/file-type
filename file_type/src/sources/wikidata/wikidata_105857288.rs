use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857288: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_288,
        source_type: SourceType::Wikidata,
        name: "Headspace Inter-Platform File Format (v1)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x52, 0x45, 0x5A, 0x00, 0x00, 0x00, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
