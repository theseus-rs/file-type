use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850557: FileFormat = FileFormat {
    id: 105_850_557,
    source_type: SourceType::Wikidata,
    name: "AmiAtlas Country data",
    extensions: &["country"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x49, 0x47, 0x41, 0x5F, 0x41, 0x54, 0x4C, 0x41, 0x53, 0x5F, 0x43,
                    0x4F, 0x55, 0x4E, 0x54, 0x52, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
