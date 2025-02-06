use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863988: FileFormat = FileFormat {
    id: 105_863_988,
    source_type: SourceType::Wikidata,
    name: "MYOB data",
    extensions: &["myo"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
