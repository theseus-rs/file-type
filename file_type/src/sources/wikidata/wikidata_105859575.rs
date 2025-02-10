use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859575: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_575,
        source_type: SourceType::Wikidata,
        name: "RealNetworks Internet Video Recording",
        extensions: &["ivr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
