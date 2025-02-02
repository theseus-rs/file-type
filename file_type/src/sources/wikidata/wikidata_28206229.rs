use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206229: FileFormat = FileFormat {
    id: 28_206_229,
    source_type: SourceType::Wikidata,
    name: "Gridded Binary",
    extensions: &["grb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
