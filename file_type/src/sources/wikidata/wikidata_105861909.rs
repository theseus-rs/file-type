use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861909: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_909,
        source_type: SourceType::Wikidata,
        name: "Magnetic Hint",
        extensions: &["hnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x61, 0x48, 0x74])],
                },
            }],
        }],
        related_formats: &[],
    },
};
