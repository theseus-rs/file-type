use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_77046033: FileFormat = FileFormat {
    id: 77_046_033,
    source_type: SourceType::Wikidata,
    name: "Extensible 3D vector graphics (VRML)",
    extensions: &["x3dv"],
    media_types: &["model/x3d-vrml"],
    signatures: &[],
    related_formats: &[],
};
