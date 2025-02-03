use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119164692: FileFormat = FileFormat {
    id: 119_164_692,
    source_type: SourceType::Wikidata,
    name: "ZoomText Configuration File",
    extensions: &["zxc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
