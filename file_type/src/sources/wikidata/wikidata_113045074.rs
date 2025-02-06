use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113045074: FileFormat = FileFormat {
    id: 113_045_074,
    source_type: SourceType::Wikidata,
    name: "PTC G-Plug Granite file",
    extensions: &["g"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
