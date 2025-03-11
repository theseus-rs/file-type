use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865404: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_404,
        source_type: SourceType::Wikidata,
        name: "Playstation Sealed Key",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x66, 0x73, 0x53, 0x4B, 0x4B, 0x65, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
