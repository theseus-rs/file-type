use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863352: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_352,
        source_type: SourceType::Wikidata,
        name: "Yamaha Midimonitor Messages",
        extensions: &["msg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x4D, 0x53, 0x47, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
