use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_60342714: FileType = FileType {
    file_format: &FileFormat {
        id: 60_342_714,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Macro-Enabled Add-In",
        extensions: &["xlam"],
        media_types: &["application/vnd.ms-excel.addin.macroEnabled.12"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
