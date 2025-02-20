use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859918: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_918,
        source_type: SourceType::Wikidata,
        name: "GameCube video",
        extensions: &["mth"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x54, 0x48, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
