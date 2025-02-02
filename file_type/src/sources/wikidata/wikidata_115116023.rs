use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115116023: FileFormat = FileFormat {
    id: 115_116_023,
    source_type: SourceType::Wikidata,
    name: "Funpaint Image File",
    extensions: &["fp2", "fun", "vic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
