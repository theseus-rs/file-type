use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850810: FileFormat = FileFormat {
    id: 105_850_810,
    source_type: SourceType::Wikidata,
    name: "Voxlap Frame Animation",
    extensions: &["kfa"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x77, 0x6C, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
