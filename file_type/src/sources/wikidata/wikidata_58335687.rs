use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58335687: FileFormat = FileFormat {
    id: 58_335_687,
    source_type: SourceType::Wikidata,
    name: "Asymetrix Toolbook File",
    extensions: &["sbk", "tbk"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x4A, 0x42, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
