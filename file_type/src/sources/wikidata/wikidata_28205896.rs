use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205896: FileFormat = FileFormat {
    id: 28_205_896,
    source_type: SourceType::Wikidata,
    name: "DESR VFF",
    extensions: &["vff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
