use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861067: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_067,
        source_type: SourceType::Wikidata,
        name: "MiAmiga Ledger data",
        extensions: &["lgr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x37, 0x32, 0x32, 0x35, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
