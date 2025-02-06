use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119164692: FileFormat = FileFormat {
    id: 119_164_692,
    source_type: SourceType::Wikidata,
    name: "ZoomText Configuration File",
    extensions: &["zxc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
