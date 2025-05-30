use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855870: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_870,
        source_type: SourceType::Wikidata,
        name: "Deployment Manager configuration",
        extensions: &["deployproj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
