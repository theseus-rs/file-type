use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857726: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_726,
        source_type: SourceType::Wikidata,
        name: "Trilo Tracker Instrument Set",
        extensions: &["is"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x53, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
