use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860802: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_802,
        source_type: SourceType::Wikidata,
        name: "RCFile format",
        extensions: &["rc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x43, 0x46, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
