use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2104918: FileFormat = FileFormat {
    id: 2_104_918,
    source_type: SourceType::Wikidata,
    name: "Portable Sound Format",
    extensions: &["minipsf", "psf", "psflib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
