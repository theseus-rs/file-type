use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_616714: FileFormat = FileFormat {
    id: 616_714,
    puid: "wikidata/616714",
    name: "Initial Graphics Exchange Specification",
    extensions: &["iges", "iges", "igs", "igs"],
    media_types: &[
        "application/iges",
        "application/iges",
        "model/iges",
        "model/iges",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
