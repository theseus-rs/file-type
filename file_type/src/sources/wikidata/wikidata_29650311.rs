use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650311: FileFormat = FileFormat {
    id: 29_650_311,
    puid: "wikidata/29650311",
    name: "POV-Ray scene description",
    extensions: &["inc", "inc", "pov", "pov"],
    media_types: &[
        "model/x-pov",
        "model/x-pov",
        "text/x-povray",
        "text/x-povray",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
