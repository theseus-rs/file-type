use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206109: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_109,
        source_type: SourceType::Wikidata,
        name: "farbfeld",
        extensions: &["ff"],
        media_types: &["application/octet-stream", "image/x-farbfeld"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x66, 0x61, 0x72, 0x62, 0x66, 0x65, 0x6C, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
