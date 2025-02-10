use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858858: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_858,
        source_type: SourceType::Wikidata,
        name: "Sound Club 2 sound bank",
        extensions: &["bnk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4E, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
