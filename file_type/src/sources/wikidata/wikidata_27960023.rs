use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960023: FileFormat = FileFormat {
    id: 27_960_023,
    source_type: SourceType::Wikidata,
    name: "Tom's lossless Audio Kompressor",
    extensions: &["tak"],
    media_types: &["audio/x-tak"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x42, 0x61, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
