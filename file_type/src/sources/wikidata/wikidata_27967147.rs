use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967147: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_147,
        source_type: SourceType::Wikidata,
        name: "Extended MOD",
        extensions: &["emd"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4D, 0x4F, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
