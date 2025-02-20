use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856989: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_989,
        source_type: SourceType::Wikidata,
        name: "Guitar Pro 6 tablature (compressed)",
        extensions: &["gpx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x43, 0x46, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
