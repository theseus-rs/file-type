use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967382: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_382,
        source_type: SourceType::Wikidata,
        name: "HMI",
        extensions: &["hmi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4D, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
