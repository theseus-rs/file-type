use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854755: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_755,
        source_type: SourceType::Wikidata,
        name: "Monkey's Audio file format",
        extensions: &["ape"],
        media_types: &["audio/x-ape"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x43, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
