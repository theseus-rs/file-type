use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206508: FileFormat = FileFormat {
    id: 28_206_508,
    source_type: SourceType::Wikidata,
    name: "Light Sheet Microscope",
    extensions: &["lsm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
