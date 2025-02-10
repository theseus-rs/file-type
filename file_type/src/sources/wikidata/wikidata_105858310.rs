use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858310: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_310,
        source_type: SourceType::Wikidata,
        name: "DOS Executable (alternate ZM id)",
        extensions: &["exe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
