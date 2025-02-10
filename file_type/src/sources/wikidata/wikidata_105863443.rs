use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863443: FileFormat = FileFormat {
    id: 105_863_443,
    source_type: SourceType::Wikidata,
    name: "Word for the Macintosh/Write for Atari ST document (v1.0)",
    extensions: &["doc", "mcw"],
    media_types: &["application/msword"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x32, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
