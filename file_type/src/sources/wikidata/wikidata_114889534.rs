use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114889534: FileFormat = FileFormat {
    id: 114_889_534,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Quilting Patch file",
    extensions: &["sqp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
