use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_672985: FileFormat = FileFormat {
    id: 672_985,
    source_type: SourceType::Wikidata,
    name: "Sun Microsystems audio file",
    extensions: &["au", "snd"],
    media_types: &["audio/basic"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x73, 0x6E, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
