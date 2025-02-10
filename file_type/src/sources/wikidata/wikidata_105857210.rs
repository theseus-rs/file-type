use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857210: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_210,
        source_type: SourceType::Wikidata,
        name: "Alpha Four Help",
        extensions: &["hlp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x34, 0x48, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
