use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130223835: FileFormat = FileFormat {
    id: 130_223_835,
    source_type: SourceType::Wikidata,
    name: "Lean 3 file format",
    extensions: &["lean"],
    media_types: &["text/x-lean", "text/x-lean3"],
    internal_signatures: &[],
    related_formats: &[],
};
