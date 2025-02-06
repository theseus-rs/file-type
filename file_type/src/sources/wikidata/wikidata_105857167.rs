use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857167: FileFormat = FileFormat {
    id: 105_857_167,
    source_type: SourceType::Wikidata,
    name: "Haiku Delta Package",
    extensions: &["hpkg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x68, 0x70, 0x6B, 0x67, 0x00, 0x50, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
