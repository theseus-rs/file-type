use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206177: FileFormat = FileFormat {
    id: 28_206_177,
    source_type: SourceType::Wikidata,
    name: "GIMP Brush",
    extensions: &["gbr", "gpb"],
    media_types: &["image/x-gimp-gbr"],
    signatures: &[],
    related_formats: &[],
};
