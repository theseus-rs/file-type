use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130285087: FileFormat = FileFormat {
    id: 130_285_087,
    source_type: SourceType::Wikidata,
    name: "Minecraft Add-ons Data Schema File",
    extensions: &["mcschema"],
    media_types: &["text/mcschema"],
    internal_signatures: &[],
    related_formats: &[],
};
