use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854207: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_207,
        source_type: SourceType::Wikidata,
        name: "SAPCAR CAR compressed archive",
        extensions: &["sar"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x43, 0x41, 0x52, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76,
                        0x65, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
