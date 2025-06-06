use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762676: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_676,
        source_type: SourceType::Wikidata,
        name: "SMS XY1 Series (old)",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x59, 0x31, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
