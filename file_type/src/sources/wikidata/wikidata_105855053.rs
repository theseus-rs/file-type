use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855053: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_053,
        source_type: SourceType::Wikidata,
        name: "ARIS Document File",
        extensions: &["adf"],
        media_types: &["x-application/adf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
