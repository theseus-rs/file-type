use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860169: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_169,
        source_type: SourceType::Wikidata,
        name: "RetroPlatform Player archive",
        extensions: &["rp9"],
        media_types: &["application/vnd.cloanto.rp9"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
