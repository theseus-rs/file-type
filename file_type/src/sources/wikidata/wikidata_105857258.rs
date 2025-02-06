use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857258: FileFormat = FileFormat {
    id: 105_857_258,
    source_type: SourceType::Wikidata,
    name: "neosat fixes",
    extensions: &["hzf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x5A, 0x46, 0x2E, 0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
