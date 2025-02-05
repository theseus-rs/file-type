use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855691: FileFormat = FileFormat {
    id: 105_855_691,
    source_type: SourceType::Wikidata,
    name: "OpenMusic Patch",
    extensions: &["omp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x20, 0x4F, 0x4D, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x48, 0x65, 0x61,
                    0x64, 0x65, 0x72, 0x20, 0x2D, 0x20, 0x53, 0x61, 0x76, 0x65, 0x64, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
