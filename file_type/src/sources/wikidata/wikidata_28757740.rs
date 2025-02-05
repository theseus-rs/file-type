use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757740: FileFormat = FileFormat {
    id: 28_757_740,
    source_type: SourceType::Wikidata,
    name: "GEM VDI Metafile",
    extensions: &["gem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
