use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851233: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_233,
        source_type: SourceType::Wikidata,
        name: "Windows 8-10 Desktop Theme (with rem)",
        extensions: &["theme"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
