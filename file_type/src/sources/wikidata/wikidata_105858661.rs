use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858661: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_661,
        source_type: SourceType::Wikidata,
        name: "Borland Graphics Interface driver (v2.x)",
        extensions: &["bgi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x6B, 0x08, 0x08])],
                },
            }],
        }],
        related_formats: &[],
    },
};
