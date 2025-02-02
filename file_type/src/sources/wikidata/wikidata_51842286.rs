use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51842286: FileFormat = FileFormat {
    id: 51_842_286,
    source_type: SourceType::Wikidata,
    name: "Microsoft PowerPoint Graphics File",
    extensions: &["ppi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
