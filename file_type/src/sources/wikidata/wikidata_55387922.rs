use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55387922: FileFormat = FileFormat {
    id: 55_387_922,
    puid: "wikidata/55387922",
    name: "Visual Molecular Dynamics file format",
    extensions: &["vmd"],
    media_types: &["chemical/x-vmd"],
    internal_signatures: &[],
    related_formats: &[],
};
