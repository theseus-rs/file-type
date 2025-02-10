use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857736: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_736,
        source_type: SourceType::Wikidata,
        name: "imgdiff patch (v1)",
        extensions: &["patch"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x4D, 0x47, 0x44, 0x49, 0x46, 0x46, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
