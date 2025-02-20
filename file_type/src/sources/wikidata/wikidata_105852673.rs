use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852673: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_673,
        source_type: SourceType::Wikidata,
        name: "Stata Data format (v114, LE)",
        extensions: &["dta"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x02, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
