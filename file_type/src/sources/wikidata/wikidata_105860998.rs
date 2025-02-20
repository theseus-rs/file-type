use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860998: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_998,
        source_type: SourceType::Wikidata,
        name: "Lecturnity Player file",
        extensions: &["lpd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x50, 0x46, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
