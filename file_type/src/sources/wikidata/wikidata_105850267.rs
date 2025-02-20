use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850267: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_267,
        source_type: SourceType::Wikidata,
        name: "StepMania Course",
        extensions: &["crs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x43, 0x4F, 0x55, 0x52, 0x53, 0x45, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
