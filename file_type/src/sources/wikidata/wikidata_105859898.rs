use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859898: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_898,
        source_type: SourceType::Wikidata,
        name: "Video Master Film/Video/Sequence",
        extensions: &["flm", "vid", "vsq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x4D, 0x41, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
