use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863305: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_305,
        source_type: SourceType::Wikidata,
        name: "Master Boot Record dump",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0xAA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
