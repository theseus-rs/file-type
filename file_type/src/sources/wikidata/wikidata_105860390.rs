use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860390: FileFormat = FileFormat {
    id: 105_860_390,
    source_type: SourceType::Wikidata,
    name: "Cloanto Amiga OS encrypted ROM",
    extensions: &["rom"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x49, 0x52, 0x4F, 0x4D, 0x54, 0x59, 0x50, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
