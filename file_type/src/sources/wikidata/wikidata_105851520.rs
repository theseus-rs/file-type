use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851520: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_520,
        source_type: SourceType::Wikidata,
        name: "CS1er debugger exported data",
        extensions: &["txt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B, 0x52, 0x65, 0x63, 0x6F, 0x72, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
