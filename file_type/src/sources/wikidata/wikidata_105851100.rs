use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851100: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_100,
        source_type: SourceType::Wikidata,
        name: "TI-Nspire PublishView document",
        extensions: &["tnsp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A, 0x54, 0x49, 0x50, 0x55, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
