use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850068: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_068,
        source_type: SourceType::Wikidata,
        name: "EasyCalc spreadsheet (v2.0)",
        extensions: &["calc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x41, 0x53, 0x59, 0x33, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
