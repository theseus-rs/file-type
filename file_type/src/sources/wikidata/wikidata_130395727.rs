use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130395727: FileFormat = FileFormat {
    id: 130_395_727,
    source_type: SourceType::Wikidata,
    name: "ODIN file format",
    extensions: &["odin"],
    media_types: &["text/odin"],
    internal_signatures: &[],
    related_formats: &[],
};
