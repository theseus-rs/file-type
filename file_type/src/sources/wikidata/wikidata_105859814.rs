use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_814,
        source_type: SourceType::Wikidata,
        name: "Yog video",
        extensions: &["yog"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x59, 0x4F, 0x47, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
