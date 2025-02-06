use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850878: FileFormat = FileFormat {
    id: 105_850_878,
    source_type: SourceType::Wikidata,
    name: "Korg Trinity/Triton sample",
    extensions: &["ksf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4D, 0x50, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
