use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858648: FileFormat = FileFormat {
    id: 105_858_648,
    source_type: SourceType::Wikidata,
    name: "Hitachi Raster Format bitmap",
    extensions: &["hrf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x44, 0x43, 0x2F, 0x4B, 0x52, 0x20, 0x52, 0x53, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
