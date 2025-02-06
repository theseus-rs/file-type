use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851369: FileFormat = FileFormat {
    id: 105_851_369,
    source_type: SourceType::Wikidata,
    name: "Speedo font Typeface Definition File (var.2)",
    extensions: &["tdf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x65, 0x76, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
