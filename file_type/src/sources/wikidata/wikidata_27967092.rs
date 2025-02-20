use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967092: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_092,
        source_type: SourceType::Wikidata,
        name: "Doom MUS",
        extensions: &["mus"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x55, 0x53, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
