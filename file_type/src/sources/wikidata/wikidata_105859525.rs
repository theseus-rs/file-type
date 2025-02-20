use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859525: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_525,
        source_type: SourceType::Wikidata,
        name: "VLBI Experiment (with rem)",
        extensions: &["vex"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
