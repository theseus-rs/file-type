use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960023: FileFormat = FileFormat {
    id: 27_960_023,
    puid: "wikidata/27960023",
    name: "Tom's lossless Audio Kompressor",
    extensions: &["tak"],
    media_types: &["audio/x-tak"],
    internal_signatures: &[InternalSignature {
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
