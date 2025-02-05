use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850868: FileFormat = FileFormat {
    id: 105_850_868,
    source_type: SourceType::Wikidata,
    name: "KSSX music format dump",
    extensions: &["kss"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x53, 0x53, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
