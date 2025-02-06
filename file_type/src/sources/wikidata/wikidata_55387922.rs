use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55387922: FileFormat = FileFormat {
    id: 55_387_922,
    source_type: SourceType::Wikidata,
    name: "Visual Molecular Dynamics file format",
    extensions: &["vmd"],
    media_types: &["chemical/x-vmd"],
    signatures: &[],
    related_formats: &[],
};
