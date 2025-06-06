use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858273: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_273,
        source_type: SourceType::Wikidata,
        name: ".NET Micro Framework PE executable",
        extensions: &["exe"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x53, 0x53, 0x70, 0x6F, 0x74, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
