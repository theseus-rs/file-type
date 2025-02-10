use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851350: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_350,
        source_type: SourceType::Wikidata,
        name: "Oric Tape image",
        extensions: &["dat", "tap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x16, 0x16, 0x16, 0x24])],
                },
            }],
        }],
        related_formats: &[],
    },
};
