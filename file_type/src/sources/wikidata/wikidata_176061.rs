use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_176061: FileFormat = FileFormat {
    id: 176_061,
    source_type: SourceType::Wikidata,
    name: "Virtual Reality Modeling Language",
    extensions: &["vrml", "wrl", "wrz"],
    media_types: &["application/x-cc3d", "model/vrml", "x-world/x-vrml"],
    signatures: &[],
    related_formats: &[],
};
