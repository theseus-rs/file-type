use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865611: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_611,
        source_type: SourceType::Wikidata,
        name: "PVM Volume format",
        extensions: &["pvm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x56, 0x4D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
