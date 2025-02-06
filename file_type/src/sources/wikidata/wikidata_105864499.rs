use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864499: FileFormat = FileFormat {
    id: 105_864_499,
    source_type: SourceType::Wikidata,
    name: "Ultimo Primo SnapShot",
    extensions: &["pss"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x53, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
