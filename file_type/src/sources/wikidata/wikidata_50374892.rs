use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50374892: FileType = FileType {
    file_format: &FileFormat {
        id: 50_374_892,
        source_type: SourceType::Wikidata,
        name: "DTS Coherent Acoustics (DCA) Audio",
        extensions: &["dts"],
        media_types: &["audio/vnd.dts"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0xFE, 0x80, 0x01, 0xFC, 0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
