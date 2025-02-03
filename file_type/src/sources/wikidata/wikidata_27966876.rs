use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966876: FileFormat = FileFormat {
    id: 27_966_876,
    source_type: SourceType::Wikidata,
    name: "2SF",
    extensions: &["2sflib", "mini2sf", "smap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
