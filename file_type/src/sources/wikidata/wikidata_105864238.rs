use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864238: FileFormat = FileFormat {
    id: 105_864_238,
    source_type: SourceType::Wikidata,
    name: "Linux PC Screen Font data (PSF2)",
    extensions: &["psf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0xB5, 0x4A, 0x86])],
            },
        }],
    }],
    related_formats: &[],
};
