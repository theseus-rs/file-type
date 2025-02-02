use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117035149: FileFormat = FileFormat {
    id: 117_035_149,
    source_type: SourceType::Wikidata,
    name: "TurboCAD for Windows ASCII File",
    extensions: &["tcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
