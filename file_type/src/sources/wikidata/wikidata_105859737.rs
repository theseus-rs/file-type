use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859737: FileFormat = FileFormat {
    id: 105_859_737,
    source_type: SourceType::Wikidata,
    name: "Hikvision DVR video",
    extensions: &["mp4"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x4B, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
