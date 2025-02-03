use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3596396: FileFormat = FileFormat {
    id: 3_596_396,
    source_type: SourceType::Wikidata,
    name: "MDX",
    extensions: &["mdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
