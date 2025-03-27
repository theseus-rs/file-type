use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_15029253: FileType = FileType {
    file_format: &FileFormat {
        id: 15_029_253,
        source_type: SourceType::Wikidata,
        name: "3D Manufacturing Format",
        extensions: &["3mf"],
        media_types: &[
            "application/vnd.ms-package.3dmanufacturing-3dmodel+xml",
            "application/vnd.ms-printing.printticket+xml",
            "model/3mf",
        ],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
