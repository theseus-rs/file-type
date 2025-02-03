use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116861020: FileFormat = FileFormat {
    id: 116_861_020,
    source_type: SourceType::Wikidata,
    name: "Disney Print Creations Project",
    extensions: &["dpc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
