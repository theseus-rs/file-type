use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857170: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_170,
        source_type: SourceType::Wikidata,
        name: "Hrust 2 packed",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x72, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
