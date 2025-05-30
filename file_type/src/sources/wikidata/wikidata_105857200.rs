use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857200: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_200,
        source_type: SourceType::Wikidata,
        name: "Hively Tracker module",
        extensions: &["hvl"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x56, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
