use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850054: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_054,
        source_type: SourceType::Wikidata,
        name: "Fine Artist Chunked format (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x48, 0x4E, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
