use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860755: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_755,
        source_type: SourceType::Wikidata,
        name: "Pokemon Randomization Quick Settings",
        extensions: &["rnqs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
