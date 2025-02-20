use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859655: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_655,
        source_type: SourceType::Wikidata,
        name: "VisiCalc spreadsheet (alt)",
        extensions: &["vc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x57, 0x31, 0x0D, 0x0A, 0x2F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
