use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857060: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_060,
        source_type: SourceType::Wikidata,
        name: "Moebius Graphics Library",
        extensions: &["glb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4C, 0x42, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
