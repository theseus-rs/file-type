use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207355: FileFormat = FileFormat {
    id: 28_207_355,
    source_type: SourceType::Wikidata,
    name: "TrueVista",
    extensions: &["vst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
