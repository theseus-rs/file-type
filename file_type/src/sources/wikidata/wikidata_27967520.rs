use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967520: FileFormat = FileFormat {
    id: 27_967_520,
    source_type: SourceType::Wikidata,
    name: "Matroska 3D Stereoscopic video",
    extensions: &["mk3d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
