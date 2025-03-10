use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857224: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_224,
        source_type: SourceType::Wikidata,
        name: "Helm multimedia book",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x45, 0x4C, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
