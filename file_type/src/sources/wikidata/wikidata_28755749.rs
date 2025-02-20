use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28755749: FileType = FileType {
    file_format: &FileFormat {
        id: 28_755_749,
        source_type: SourceType::Wikidata,
        name: "FDF",
        extensions: &["fdf"],
        media_types: &["application/vnd.fdf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x46, 0x44, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
