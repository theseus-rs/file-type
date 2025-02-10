use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852980: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_980,
        source_type: SourceType::Wikidata,
        name: "Mocha Snapshot",
        extensions: &["snp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC, 0xED])],
                },
            }],
        }],
        related_formats: &[],
    },
};
