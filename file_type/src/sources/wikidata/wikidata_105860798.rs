use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860798: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_798,
        source_type: SourceType::Wikidata,
        name: "Allods 2 Rage Of Mages game data archive",
        extensions: &["res"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x26, 0x59, 0x41, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
