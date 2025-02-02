use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51839187: FileFormat = FileFormat {
    id: 51_839_187,
    source_type: SourceType::Wikidata,
    name: "NAP Metafile",
    extensions: &["nap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
