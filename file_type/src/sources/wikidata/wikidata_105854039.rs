use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854039: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_039,
        source_type: SourceType::Wikidata,
        name: "RCA digital voice recorder audio",
        extensions: &["voc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5F, 0x56, 0x4F, 0x43, 0x5F, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
