use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859971: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_971,
        source_type: SourceType::Wikidata,
        name: "Fast Search and Transfer video",
        extensions: &["fvt"],
        media_types: &["video/vnd.fvt"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x56, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
