use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860277: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_277,
        source_type: SourceType::Wikidata,
        name: "RASTER Music Tracker Instrument",
        extensions: &["rti"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x54, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
