use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849612: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_612,
        source_type: SourceType::Wikidata,
        name: "Chromoium delta update format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x72, 0x41, 0x55])],
                },
            }],
        }],
        related_formats: &[],
    },
};
