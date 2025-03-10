use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857049: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_049,
        source_type: SourceType::Wikidata,
        name: "Gzz saved data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x5A, 0x5A, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
