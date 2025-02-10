use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867402: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_402,
        source_type: SourceType::Wikidata,
        name: "NSIS script (with rem)",
        extensions: &["nsi"],
        media_types: &["text/plain", "text/x-nsis"],
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
