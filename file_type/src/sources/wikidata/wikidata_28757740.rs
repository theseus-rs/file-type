use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757740: FileFormat = FileFormat {
    id: 28_757_740,
    source_type: SourceType::Wikidata,
    name: "GEM VDI Metafile",
    extensions: &["gem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
