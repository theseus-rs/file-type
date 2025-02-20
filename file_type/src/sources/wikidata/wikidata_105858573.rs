use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858573: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_573,
        source_type: SourceType::Wikidata,
        name: "BIF bitmap ASCII info",
        extensions: &["bif"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x49, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
