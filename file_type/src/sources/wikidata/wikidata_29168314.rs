use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29168314: FileType = FileType {
    file_format: &FileFormat {
        id: 29_168_314,
        source_type: SourceType::Wikidata,
        name: "Microsoft Archive",
        extensions: &["mar"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x52, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
