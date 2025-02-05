use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852701: FileFormat = FileFormat {
    id: 105_852_701,
    source_type: SourceType::Wikidata,
    name: "SNX Snapshot",
    extensions: &["snx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x53, 0x4E, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
