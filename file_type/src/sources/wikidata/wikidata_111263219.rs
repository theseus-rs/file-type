use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111263219: FileFormat = FileFormat {
    id: 111_263_219,
    source_type: SourceType::Wikidata,
    name: "DCM module",
    extensions: &["dcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
