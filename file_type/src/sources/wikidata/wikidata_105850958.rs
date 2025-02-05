use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850958: FileFormat = FileFormat {
    id: 105_850_958,
    source_type: SourceType::Wikidata,
    name: "TeXnicCenter Project",
    extensions: &["tcp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x49, 0x6E, 0x66, 0x6F, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
