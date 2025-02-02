use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129326955: FileFormat = FileFormat {
    id: 129_326_955,
    source_type: SourceType::Wikidata,
    name: "GDScript source code file",
    extensions: &["gd"],
    media_types: &["application/x-gdscript", "text/x-gdscript"],
    internal_signatures: &[],
    related_formats: &[],
};
