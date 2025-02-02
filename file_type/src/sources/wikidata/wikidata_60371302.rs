use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60371302: FileFormat = FileFormat {
    id: 60_371_302,
    source_type: SourceType::Wikidata,
    name: "Microsoft PowerPoint Macro-Enabled Show",
    extensions: &["ppsm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
