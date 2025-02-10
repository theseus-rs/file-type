use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858317: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_317,
        source_type: SourceType::Wikidata,
        name: "Melco embroidery design",
        extensions: &["exp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
