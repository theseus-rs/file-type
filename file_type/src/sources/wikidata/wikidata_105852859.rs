use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852859: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_859,
        source_type: SourceType::Wikidata,
        name: "Sequential Vibes Music",
        extensions: &["svm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x45, 0x51, 0x56, 0x49, 0x42, 0x45, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
