use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96054590: FileFormat = FileFormat {
    id: 96_054_590,
    source_type: SourceType::Wikidata,
    name: "Macromolecular Crystallographic Information File",
    extensions: &["mmcif"],
    media_types: &["chemical/x-mmcif"],
    internal_signatures: &[],
    related_formats: &[],
};
