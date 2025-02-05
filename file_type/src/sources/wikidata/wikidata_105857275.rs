use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857275: FileFormat = FileFormat {
    id: 105_857_275,
    source_type: SourceType::Wikidata,
    name: "Altera Qsys Handoff Info",
    extensions: &["hiof"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x49, 0x4F, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
