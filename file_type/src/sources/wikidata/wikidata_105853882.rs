use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853882: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_882,
        source_type: SourceType::Wikidata,
        name: "STK Azimuth-Elevation Mask format",
        extensions: &["aem"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x74, 0x6B, 0x2E, 0x76, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
