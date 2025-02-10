use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859627: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_627,
        source_type: SourceType::Wikidata,
        name: "Visual Studio Setup and Deployment Project",
        extensions: &["vdproj"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x22, 0x44, 0x65, 0x70, 0x6C, 0x6F, 0x79, 0x50, 0x72, 0x6F, 0x6A, 0x65,
                        0x63, 0x74, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
