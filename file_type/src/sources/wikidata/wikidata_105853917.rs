use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853917: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_917,
        source_type: SourceType::Wikidata,
        name: "ZipGenius encrypted compressed archive",
        extensions: &["czip"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x5A, 0x49, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
