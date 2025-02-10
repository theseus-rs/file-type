use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967347: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_347,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score",
        extensions: &["si7", "sib"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0F, 0x53, 0x49, 0x42, 0x45, 0x4C, 0x49, 0x55, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
