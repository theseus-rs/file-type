use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119217819: FileFormat = FileFormat {
    id: 119_217_819,
    source_type: SourceType::Wikidata,
    name: "QuickBooks Portable Company File",
    extensions: &["qbm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
