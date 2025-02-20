use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863010: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_010,
        source_type: SourceType::Wikidata,
        name: "PI Image Patterns",
        extensions: &["motivi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x4F, 0x52, 0x4D, 0x50, 0x54, 0x50, 0x49,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
