use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919081: FileFormat = FileFormat {
    id: 28_919_081,
    source_type: SourceType::Wikidata,
    name: "Adobe Premiere Batch List",
    extensions: &["pbl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
