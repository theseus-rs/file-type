use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967532: FileFormat = FileFormat {
    id: 27_967_532,
    source_type: SourceType::Wikidata,
    name: "DVD Information File",
    extensions: &["bup", "ifo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
