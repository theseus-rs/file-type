use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857077: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_077,
        source_type: SourceType::Wikidata,
        name: "PlayStation RSD 3D Group (gen)",
        extensions: &["grp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x47, 0x52, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
