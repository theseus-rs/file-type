use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857407: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_407,
        source_type: SourceType::Wikidata,
        name: "jalbum image info",
        extensions: &["jpx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
