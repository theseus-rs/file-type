use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851864: FileFormat = FileFormat {
    id: 105_851_864,
    source_type: SourceType::Wikidata,
    name: "S10 WebAlbums project",
    extensions: &["s10w"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x6C, 0x62, 0x75, 0x6D, 0x73, 0x4C, 0x69, 0x73, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
