use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113494624: FileFormat = FileFormat {
    id: 113_494_624,
    puid: "wikidata/113494624",
    name: "Persuasion Presentation Interchange File",
    extensions: &["prf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
