use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858285: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_285,
        source_type: SourceType::Wikidata,
        name: "Ease document (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x50, 0x45, 0x52, 0x41, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
