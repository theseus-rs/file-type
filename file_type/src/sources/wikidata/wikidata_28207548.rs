use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207548: FileFormat = FileFormat {
    id: 28_207_548,
    source_type: SourceType::Wikidata,
    name: "Xerox EDMICS-RLC",
    extensions: &["rlc"],
    media_types: &["image/vnd.fujixerox.edmics-rlc"],
    internal_signatures: &[],
    related_formats: &[],
};
