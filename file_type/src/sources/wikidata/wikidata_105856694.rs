use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856694: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_694,
        source_type: SourceType::Wikidata,
        name: "Unified Scripture Format XML",
        extensions: &["usx"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
