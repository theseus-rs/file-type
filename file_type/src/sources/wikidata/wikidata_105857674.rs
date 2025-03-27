use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857674: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_674,
        source_type: SourceType::Wikidata,
        name: "imgdiff patch (v2)",
        extensions: &["patch"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x4D, 0x47, 0x44, 0x49, 0x46, 0x46, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
