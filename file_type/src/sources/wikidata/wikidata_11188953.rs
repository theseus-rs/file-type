use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_11188953: FileType = FileType {
    file_format: &FileFormat {
        id: 11_188_953,
        source_type: SourceType::Wikidata,
        name: "Astrotite",
        extensions: &["afa"],
        media_types: &["application/x-astrotite-afa"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x53, 0x54, 0x56, 0x53, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
