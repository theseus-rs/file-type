use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859601: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_601,
        source_type: SourceType::Wikidata,
        name: "Lenel video",
        extensions: &["lnr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x4E, 0x52, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
