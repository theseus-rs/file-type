use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_176061: FileFormat = FileFormat {
    id: 176_061,
    puid: "wikidata/176061",
    name: "Virtual Reality Modeling Language",
    extensions: &[
        "vrml", "vrml", "vrml", "vrml", "vrml", "vrml", "wrl", "wrl", "wrl", "wrl", "wrl", "wrl",
        "wrz", "wrz", "wrz", "wrz", "wrz", "wrz",
    ],
    media_types: &[
        "application/x-cc3d",
        "application/x-cc3d",
        "application/x-cc3d",
        "application/x-cc3d",
        "application/x-cc3d",
        "application/x-cc3d",
        "model/vrml",
        "model/vrml",
        "model/vrml",
        "model/vrml",
        "model/vrml",
        "model/vrml",
        "x-world/x-vrml",
        "x-world/x-vrml",
        "x-world/x-vrml",
        "x-world/x-vrml",
        "x-world/x-vrml",
        "x-world/x-vrml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
