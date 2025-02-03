use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_55387922: FileFormat = FileFormat {
    id: 55_387_922,
    source_type: SourceType::Wikidata,
    name: "Visual Molecular Dynamics file format",
    extensions: &["vmd"],
    media_types: &["chemical/x-vmd"],
    internal_signatures: &[],
    related_formats: &[],
};
