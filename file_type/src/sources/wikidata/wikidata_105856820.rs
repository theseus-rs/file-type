use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856820: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_820,
        source_type: SourceType::Wikidata,
        name: "Guitar Pro 6 tablature (non compressed)",
        extensions: &["gpx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x43, 0x46, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
