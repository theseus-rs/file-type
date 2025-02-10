use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863393: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_393,
        source_type: SourceType::Wikidata,
        name: "Meyer/Glass Interactive game data Format",
        extensions: &["mgf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x47, 0x46, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
