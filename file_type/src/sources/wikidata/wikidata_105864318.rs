use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864318: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_318,
        source_type: SourceType::Wikidata,
        name: "RadPHP Project",
        extensions: &["phprj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x48, 0x50, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
