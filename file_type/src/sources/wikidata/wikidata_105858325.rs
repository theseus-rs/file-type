use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858325: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_325,
        source_type: SourceType::Wikidata,
        name: "Extended-PSG chiptune format",
        extensions: &["epsg", "psg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x50, 0x53, 0x47, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
