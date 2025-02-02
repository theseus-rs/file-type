use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28919081: FileFormat = FileFormat {
    id: 28_919_081,
    source_type: SourceType::Wikidata,
    name: "Adobe Premiere Batch List",
    extensions: &["pbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
