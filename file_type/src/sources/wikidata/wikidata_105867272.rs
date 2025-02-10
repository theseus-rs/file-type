use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867272: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_272,
        source_type: SourceType::Wikidata,
        name: "Legend text",
        extensions: &["nbi"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x40])],
                },
            }],
        }],
        related_formats: &[],
    },
};
