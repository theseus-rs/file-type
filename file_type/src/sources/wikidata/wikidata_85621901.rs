use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85621901: FileType = FileType {
    file_format: &FileFormat {
        id: 85_621_901,
        source_type: SourceType::Wikidata,
        name: "PFS:First Choice Graph",
        extensions: &["gra"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x52, 0x41, 0x42,
                        0x42, 0x49, 0x54, 0x47, 0x52, 0x41, 0x50, 0x48,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
