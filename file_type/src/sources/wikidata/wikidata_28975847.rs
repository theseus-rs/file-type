use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975847: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_847,
        source_type: SourceType::Wikidata,
        name: "Neutral ASCII File Format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0x66, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
