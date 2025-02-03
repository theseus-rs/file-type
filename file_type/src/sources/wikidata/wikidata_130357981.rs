use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130357981: FileFormat = FileFormat {
    id: 130_357_981,
    source_type: SourceType::Wikidata,
    name: "MoonScript source code file",
    extensions: &["moon"],
    media_types: &["application/x-moonscript", "text/x-moonscript"],
    internal_signatures: &[],
    related_formats: &[],
};
