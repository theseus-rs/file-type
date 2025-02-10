use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852924: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_924,
        source_type: SourceType::Wikidata,
        name: "Softdisk Help Library format",
        extensions: &["abs", "cat", "shl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4D, 0x50, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
