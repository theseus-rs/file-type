use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855308: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_308,
        source_type: SourceType::Wikidata,
        name: "SimCity 3000 Freshness Score",
        extensions: &["fsc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x37,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
