use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852683: FileFormat = FileFormat {
    id: 105_852_683,
    source_type: SourceType::Wikidata,
    name: "WinSong Composer Song",
    extensions: &["sng"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x08, 0x08, 0x30, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
