use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852112: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_112,
        source_type: SourceType::Wikidata,
        name: "Sound Club module",
        extensions: &["sn"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4E, 0x47, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
