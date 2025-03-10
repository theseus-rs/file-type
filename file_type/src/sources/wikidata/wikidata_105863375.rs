use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863375: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_375,
        source_type: SourceType::Wikidata,
        name: "PS2 code overlay",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x57, 0x6F, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
