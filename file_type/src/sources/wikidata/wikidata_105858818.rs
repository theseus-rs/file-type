use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858818: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_818,
        source_type: SourceType::Wikidata,
        name: "Nokia RAW bitmap",
        extensions: &["nrw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x4F, 0x4B, 0x49, 0x41, 0x52, 0x41, 0x57,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
