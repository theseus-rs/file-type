use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111263219: FileFormat = FileFormat {
    id: 111_263_219,
    source_type: SourceType::Wikidata,
    name: "DCM module",
    extensions: &["dcm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
