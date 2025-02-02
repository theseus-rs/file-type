use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61963304: FileFormat = FileFormat {
    id: 61_963_304,
    source_type: SourceType::Wikidata,
    name: "Microsoft Front Page Binary Tree Index",
    extensions: &["btr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
