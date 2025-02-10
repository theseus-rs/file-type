use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860885: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_885,
        source_type: SourceType::Wikidata,
        name: "Respawn Entertainment game data archive (generic)",
        extensions: &["rpak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x50, 0x61, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
