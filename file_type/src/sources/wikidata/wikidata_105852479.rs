use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852479: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_479,
        source_type: SourceType::Wikidata,
        name: "SigmaNEST Shape",
        extensions: &["shp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x48, 0x41, 0x50, 0x45, 0x20, 0x20, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
