use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849842: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_842,
        source_type: SourceType::Wikidata,
        name: "MSI/Accelrys Cerius II",
        extensions: &["cer", "msi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
