use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867580: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_580,
        source_type: SourceType::Wikidata,
        name: "Nero ISO Compilation",
        extensions: &["nri"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0E, 0x4E, 0x65, 0x72, 0x6F, 0x49, 0x53, 0x4F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
