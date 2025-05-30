use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206242: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_242,
        source_type: SourceType::Wikidata,
        name: "GX1",
        extensions: &["gx1"],
        media_types: &["application/octet-stream", "image/x-gx1"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFA, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
