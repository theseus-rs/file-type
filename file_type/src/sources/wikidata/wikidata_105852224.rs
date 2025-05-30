use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852224: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_224,
        source_type: SourceType::Wikidata,
        name: "PolyPlot Symbol",
        extensions: &["sym"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x13, 0x08, 0x08, 0x08, 0x50, 0x6F, 0x6C, 0x79, 0x70, 0x6C, 0x6F,
                        0x74, 0x20, 0x53, 0x79, 0x6D, 0x62, 0x6F, 0x6C, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
