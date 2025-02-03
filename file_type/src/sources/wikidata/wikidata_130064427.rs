use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130064427: FileFormat = FileFormat {
    id: 130_064_427,
    source_type: SourceType::Wikidata,
    name: "Koka file format",
    extensions: &["kk"],
    media_types: &["text/x-koka"],
    internal_signatures: &[],
    related_formats: &[],
};
