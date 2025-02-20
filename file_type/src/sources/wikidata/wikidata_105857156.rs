use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857156: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_156,
        source_type: SourceType::Wikidata,
        name: "Hammer compressed",
        extensions: &["hmr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x6D, 0x72, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
