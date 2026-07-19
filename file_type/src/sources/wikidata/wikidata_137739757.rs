use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_137739757: FileType = FileType {
    file_format: &FileFormat {
        id: 137_739_757,
        source_type: SourceType::Wikidata,
        name: "Jamdac chiptune file format",
        extensions: &["jamdac"],
        media_types: &["audio/x-jamdac"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x41, 0x4D, 0x44, 0x41, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
