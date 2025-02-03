use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130542831: FileFormat = FileFormat {
    id: 130_542_831,
    source_type: SourceType::Wikidata,
    name: "Pug file format",
    extensions: &["jade", "pug"],
    media_types: &["text/x-jade", "text/x-pug"],
    internal_signatures: &[],
    related_formats: &[],
};
