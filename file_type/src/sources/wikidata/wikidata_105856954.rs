use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856954: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_954,
        source_type: SourceType::Wikidata,
        name: "GenBank sequence record",
        extensions: &["gb", "gbk", "genbank", "gp"],
        media_types: &["chemical/seq-na-genbank"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x4F, 0x43, 0x55, 0x53, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
