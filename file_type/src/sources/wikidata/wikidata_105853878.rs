use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853878: FileFormat = FileFormat {
    id: 105_853_878,
    source_type: SourceType::Wikidata,
    name: "7-Zip compressed archive (v0.4)",
    extensions: &["7z"],
    media_types: &["application/x-7z-compressed"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C, 0x00, 0x04,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
