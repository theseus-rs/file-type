use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850894: FileFormat = FileFormat {
    id: 105_850_894,
    source_type: SourceType::Wikidata,
    name: "KGen98 Save State",
    extensions: &["kss"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x53, 0x53, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
