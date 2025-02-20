use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854676: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_676,
        source_type: SourceType::Wikidata,
        name: "Quake archive",
        extensions: &["pak"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x43, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
