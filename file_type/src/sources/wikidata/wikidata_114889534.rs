use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114889534: FileFormat = FileFormat {
    id: 114_889_534,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Quilting Patch file",
    extensions: &["sqp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
