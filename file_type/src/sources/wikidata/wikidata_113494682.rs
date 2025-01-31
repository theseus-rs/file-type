use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113494682: FileFormat = FileFormat {
    id: 113_494_682,
    puid: "wikidata/113494682",
    name: "Persuasion Auto-Template Interchange File",
    extensions: &["atf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
